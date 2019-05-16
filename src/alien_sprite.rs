use super::{rand_int, Resources};
use mengine::engine::{
    Point, Rect, Resource, Sprite, SpriteExt, BA_DIE, SA_ADDSPRITE, SPRITEACTION,
};
use std::cell::RefCell;
use std::rc::Rc;

//外星人精灵扩展

pub struct AlienSprite {
    pub resources: Rc<Resources>,
    pub difficulty: Rc<RefCell<i32>>,
}

impl SpriteExt for AlienSprite {
    fn update(&self, sprite_action: SPRITEACTION) -> SPRITEACTION {
        //检查精灵是否要发射子弹
        match rand_int(0, *self.difficulty.borrow() / 2) {
            0 => sprite_action | SA_ADDSPRITE,
            _ => sprite_action,
        }
    }

    fn add_sprite(&self, sprite: &Sprite) -> Sprite {
        //创建一个新的子弹精灵
        let bounds = Rect::new(0.0, 0.0, 640.0, 410.0);
        let pos = sprite.position();
        let mut velocity = Point { x: 0.0, y: 0.0 };

        let missile_bitmap = match sprite.name() {
            "blobbo" => {
                velocity.y = 7.0;
                self.resources.img_bmissile.clone()
            }
            "jelly" => {
                velocity.y = 5.0;
                self.resources.img_jmissile.clone()
            }
            _ => {
                velocity.y = 3.0;
                self.resources.img_tmissile.clone()
            }
        };
        let mut sub_sprite = Sprite::with_bounds_action(
            String::from("amissile"),
            Resource::Static(missile_bitmap),
            bounds,
            BA_DIE,
        );
        sub_sprite.set_velocity(velocity.x, velocity.y);
        sub_sprite.set_position(pos.left + sprite.width() / 2.0, pos.bottom);
        sub_sprite
    }
}
