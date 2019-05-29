use super::rand_int;
use mengine::*;
use std::cmp;

//星星背景

pub struct StarryBackground {
    width: f64,
    height: f64,
    num_stars: usize,
    twink_delay: i32,
    stars: [Point; 100],
    star_colors: [(u8, u8, u8); 100],
}

impl StarryBackground {
    pub fn new(width: f64, height: f64, num_stars: usize, twink_delay: i32) -> StarryBackground {
        let num_stars = cmp::min(num_stars, 100);
        //创建星星
        let mut stars = [Point { x: 0., y: 0. }; 100];
        let mut star_colors = [(255, 255, 255); 100];
        for i in 0..num_stars {
            stars[i].x = rand_int(0, width as i32) as f64;
            stars[i].y = rand_int(0, height as i32) as f64;
            star_colors[i] = (128, 128, 128);
        }
        StarryBackground {
            width,
            height,
            num_stars,
            twink_delay,
            stars,
            star_colors,
        }
    }

    pub fn default(width: f64, height: f64) -> StarryBackground {
        StarryBackground::new(width, height, 100, 50)
    }

    pub fn update(&mut self) {
        //随机改变星星的颜色以使其闪烁
        for i in 0..self.num_stars {
            if rand_int(0, self.twink_delay) == 0 {
                let color = rand_int(0, 256) as u8;
                self.star_colors[i] = (color, color, color);
            }
        }
    }

    pub fn draw(&self, g: &mut Graphics) {
        //绘制纯黑色背景
        g.fill_rect(&[0, 0, 0, 255], 0., 0., self.width, self.height);
        //绘制星星
        for i in 0..self.num_stars {
            g.fill_rect(
                &[
                    self.star_colors[i].0,
                    self.star_colors[i].1,
                    self.star_colors[i].2,
                    255,
                ],
                self.stars[i].x,
                self.stars[i].y,
                1.,
                1.,
            );
        }
    }
}
