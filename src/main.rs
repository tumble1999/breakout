use bevy::{
	prelude::*,
	//input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    window::CursorMoved,
};

// Resources
// Components
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity {
	direction: Vec3,
	speed: f32,
}


#[derive(Component)]
struct BoundingBox{
	left:f32,
	right:f32,
	bottom:f32,
	top:f32,
}

// Systems
fn setup(mut commands: Commands) {
	// Camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// Ball
	let ball_size = 10.;
	commands.spawn_bundle(SpriteBundle {
		transform:Transform {
			//translation:Vec3::new(-ball_size/2.,-ball_size/2.,0.),
			translation:Vec3::new(0.,0.,1.),
			..Default::default()
		},
		sprite: Sprite{
			color:Color::WHITE,
			custom_size:Some(Vec2::new(
				ball_size,
				ball_size
			)),
			..Default::default()
		},
		..Default::default()
	}).insert(Ball).insert(Velocity{
		direction:Vec3::new(1.,-1.,0.),
		speed: 500.
	});



	// Paddle
	commands.spawn_bundle(SpriteBundle {
		transform:Transform {
			//translation:Vec3::new(-ball_size/2.,-ball_size/2.,0.),
			translation:Vec3::new(0.,-100.,1.),
			..Default::default()
		},
		sprite: Sprite{
			color:Color::WHITE,
			custom_size:Some(Vec2::new(
				100.,
				20.
			)),
			..Default::default()
		},
		..Default::default()
	}).insert(Paddle).insert(BoundingBox{
		left:-50.,
		bottom:-10.,
		right:50.,
		top:10.,
	});

}


fn ball_collision(mut query:Query<(&Transform, &mut Velocity, With<Ball>)>, windows: Res<Windows>) {
	let ball = query.single_mut();
	let transform = ball.0;
	let mut velocity = ball.1;
	let window = windows.get_primary().unwrap();
	let window_radius = Vec2::new(window.width()/2.,window.height()/2.);

	/*
	If the Position is positive we want it to go left
	if the position is negative we want it to go right

	This works off of the inverse of
	+ * + = +
	+ * - = -
	- * + = -
	- * - = +
	*/
	let dir_mult = transform.translation * velocity.direction;

	if transform.translation.x > window_radius.x ||
		transform.translation.x < -window_radius.x {
		//velocity.direction.x *= -dir_mult.x.signum();

		//simplified: -v*abs(vp)/vp  =>  -abs(vp)/p
		velocity.direction.x = -dir_mult.x.abs()/transform.translation.x;

	}

	if transform.translation.y > window_radius.y ||
		transform.translation.y < -window_radius.y {
		//velocity.direction.y *= -dir_mult.y.signum();

		//simplified: -v*abs(vp)/vp  =>  -abs(vp)/p
		velocity.direction.y = -dir_mult.y.abs()/transform.translation.y;
	}

	//info!("xy:{}	dir_mult:{}	dir:	speed:{}",transform.translation.x, dir_mult, velocity.direction,velocity.speed);
	//info!("dir_mult: {} ",dir_mult);
       // info!("{:?} {:?}", transform.translation,velocity);

}

fn collision(mut mov_query: Query<(&Transform, &mut Velocity)>, col_query:Query<(&Transform, &BoundingBox)>) {
	for mov_obj in mov_query.iter_mut() {
		// Get the transform and velocity of moving object
		let mov_tf = mov_obj.0;
		let mov_pos = mov_tf.translation;
		let mut mov_vel = mov_obj.1;

		for col_obj in col_query.iter() {
			// get the transform and bounds of collision object
			let col_tf = col_obj.0;
			let col_bounds = col_obj.1;
			let col_pos = col_tf.translation;
			let col_left = col_pos.x + col_bounds.left;
			let col_right = col_pos.x + col_bounds.right;
			let col_bottom = col_pos.y + col_bounds.bottom;
			let col_top = col_pos.y + col_bounds.top;

			// Do a similar thing as i did in ball_collision()
			/*
			(L)left:5 (R)right 8
			(B)ball x: 6

			5<6<8
			B-L<R-B
			(A)Distance to left: 6-5=1 (B)Distance to right:8-6=2
			A>B
			A-B (if its positive go )

			*/
			let low = Vec3::new(col_left,col_bottom,0.);
			let high = Vec3::new(col_right,col_top,0.);
			let dir_mult = -(low+high-2.*mov_pos)*mov_vel.direction;

			let in_left = col_left < mov_pos.x;
			let in_right = mov_pos.x < col_right;
			let in_bottom = col_bottom < mov_pos.y;
			let in_top = mov_pos.y < col_top;
			let hor_infinite = col_right < col_left;
			let vir_infinite = col_top < col_bottom;

			//info!("scale: {:?}",col_tf.scale);

			// if collided
			if (hor_infinite||in_left&&in_right) && (vir_infinite||in_top&&in_bottom) {
				info!("L X R: {} {} {}",col_left,mov_pos.x,col_right);
				info!("B Y T: {} {} {}",col_bottom,mov_pos.y,col_top);
				//info!("in_left_right: {} {}",in_left,in_right);
				//info!("in_bottom_top: {} {}",in_bottom,in_top);
				if
					if hor_infinite {
						in_left || in_right
					} else {
						in_left && in_right
					}
					{
						if mov_vel.direction.x==0. {
							mov_vel.direction.x=0.5;
						}
						mov_vel.direction.x = -dir_mult.x.abs()/mov_pos.x;
					}
				if
					if vir_infinite {
						in_top || in_bottom
					} else {
						in_top && in_bottom
					}
					{
						if mov_vel.direction.y==0. {
							mov_vel.direction.y=0.5;
						}
						mov_vel.direction.y = -dir_mult.y.abs()/mov_pos.y;
					}
			}


			// // First check if the collision object wraps around the outside or is contained
			// // then check if the moving object is in bounds of the collision object
			// if if col_right > col_left { col_left < mov_tf.translation.x && mov_tf.translation.x < col_right } else { col_left < mov_tf.translation.x || mov_tf.translation.x < col_right } {
			// 	// Flip the velocity depending on the current velocity
			// 	mov_vel.direction.x = -dir_mult.x.abs()/mov_tf.translation.x;
			// }

			// // First check if the collision object wraps around the outside or is contained
			// // then check if the moving object is in bounds of the collision object
			// if if col_top > col_bottom { col_bottom < mov_tf.translation.y && mov_tf.translation.y < col_top } else { col_bottom < mov_tf.translation.y || mov_tf.translation.y < col_top } {
			// 	// Flip the velocity depending on the current velocity
			// 	mov_vel.direction.y = -dir_mult.y.abs()/mov_tf.translation.y;
			// }
		}
	}
}


fn ball_movement(mut query:Query<(&mut Transform, &Velocity, With<Ball>)>, time: Res<Time>) {
	let ball = query.single_mut();
	let mut transform = ball.0;
	let velocity = ball.1;

	transform.translation += velocity.direction.normalize_or_zero()*velocity.speed*time.delta_seconds();
}

fn paddle_movement(mut query:Query<(&mut Transform, With<Paddle>)>, windows: Res<Windows>,mut cursor_moved_events: EventReader<CursorMoved>) {
	let paddle =query.single_mut();
	let mut transform = paddle.0;
	// Window
	let window = windows.get_primary().unwrap();

	transform.translation.y = 100.-window.height()/2.;


	for event in cursor_moved_events.iter() {
		transform.translation.x = event.position.x-window.width()/2.;
		//transform.translation.y = event.position.y-window.height()/2.;
	}
}


// fn print_mouse_events_system(
//     mut mouse_button_input_events: EventReader<MouseButtonInput>,
//     mut mouse_motion_events: EventReader<MouseMotion>,
//     mut cursor_moved_events: EventReader<CursorMoved>,
//     mut mouse_wheel_events: EventReader<MouseWheel>,
// ) {
//     for event in mouse_button_input_events.iter() {
//         info!("{:?}", event);
//     }

//     for event in mouse_motion_events.iter() {
//         info!("{:?}", event);
//     }

//     for event in cursor_moved_events.iter() {
//         info!("{:?}", event);
//     }

//     for event in mouse_wheel_events.iter() {
//         info!("{:?}", event);
//     }
// }

fn main() {
    App::new()
	.add_plugins(DefaultPlugins)
	.add_startup_system(setup.system())
	.add_system(ball_collision.system())
	.add_system(paddle_movement.system())
	//.add_system(print_mouse_events_system.system())
	.add_system(collision.system())
	.add_system(ball_movement.system())
	.run();
}
