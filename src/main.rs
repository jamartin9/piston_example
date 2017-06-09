#[macro_use]
extern crate clap;
extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate fps_counter;

fn main() {
    use fps_counter::FPSCounter;
    use std::rc::Rc;
    use piston_window::*;
    use sprite::*;
    use ai_behavior::{Action,
                      Sequence,
                      //    Wait,
                      WaitForever,
                      While};
    use clap::App;

    // get cli args
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let width = matches.value_of("width").unwrap_or("400").parse().expect("expected a number");
    let height = matches.value_of("height").unwrap_or("400").parse().expect("expected a number");

    let mut window: PistonWindow = WindowSettings::new("piston game", (width, height))
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    // get font assets
    let font = &assets.join("FiraCode-Regular-modified.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    // create sprites
    let background_tex = Rc::new(Texture::from_path(&mut window.factory,
                                         assets.join("figure_1.png"),
                                         Flip::None,
                                         &TextureSettings::new())
        .unwrap());
    let mut background = Sprite::from_texture(background_tex.clone());
    background.set_position(width as f64 / 2.0, height as f64 / 2.0);

    let player_tex = Rc::new(Texture::from_path(&mut window.factory,
                                         assets.join("rust.png"),
                                         Flip::None,
                                         &TextureSettings::new())
        .unwrap());
    let mut sprite = Sprite::from_texture(player_tex.clone());
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    let player_id = background.add_child(sprite);

    // create scene
    let mut scene = Scene::new();
    let background_id = scene.add_child(background);

    // create animations
    let blink = While(Box::new(WaitForever),
                      vec![Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
                           Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0))))]);
    let left_anim = Sequence(vec![Action(Ease(EaseFunction::BounceOut,
                                              Box::new(MoveBy(1.0, -100.0, 0.0))))]);
    let right_anim = Sequence(vec![Action(Ease(EaseFunction::BounceOut,
                                               Box::new(MoveBy(1.0, 100.0, 0.0))))]);
    let up_anim = Sequence(vec![Action(Ease(EaseFunction::BounceOut,
                                            Box::new(MoveBy(1.0, 0.0, -100.0))))]);
    let down_anim = Sequence(vec![Action(Ease(EaseFunction::BounceOut,
                                              Box::new(MoveBy(1.0, 0.0, 100.0))))]);

    // init scene setup
    scene.run(player_id, &blink);
    scene.toggle(player_id, &blink);

    // fps setup
    let mut fps_counter = FPSCounter::new();
    let mut show_fps = false;

    while let Some(e) = window.next() {

        let fps = fps_counter.tick();
        scene.event(&e);

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::F => {
                    // toggle fps
                    if show_fps {
                        show_fps = false;
                    } else {
                        show_fps = true;
                    }
                }
                Key::A => scene.run(player_id, &left_anim),
                Key::D => scene.run(player_id, &right_anim),
                Key::S => scene.run(player_id, &down_anim),
                Key::W => scene.run(player_id, &up_anim),
                Key::Space => scene.toggle(player_id, &blink),
                _ => println!("Unregistered keyboard key '{:?}'", key),
            }
        }

        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);

            // show fps
            if show_fps {
                let fps_display_string = &*fps.to_string();
                let transform = c.transform.trans(10.0, 30.0);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                    .draw(fps_display_string, &mut glyphs, &c.draw_state, transform, g);
            }

        });

    }
}
