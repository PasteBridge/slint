// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// 生成 100 条带 mock 图片的 ListView 条目，用于测试滚动帧率。

use std::rc::Rc;

use slint::{Image, ModelRc, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel};

slint::include_modules!();

// 根据索引号生成一张简单的彩色 mock 图片（径向/水平渐变）。
// 这样每张图都有不同的颜色，便于观察 ListView 的虚拟化行为。
fn make_mock_image(index: u32) -> Image {
    const W: u32 = 128;
    const H: u32 = 128;
    let mut pixels = SharedPixelBuffer::<Rgba8Pixel>::new(W, H);

    // 基于 index 生成 HSV 风格的颜色基调。
    let hue = (index % 360) as f32;
    let (base_r, base_g, base_b) = hsv_to_rgb(hue, 0.75f32, 0.95f32);

    let slice = pixels.make_mut_slice();
    for y in 0..H {
        for x in 0..W {
            // 圆形渐变 + 对角条纹，使每张图视觉上不同。
            let dx = (x as f32 - (W as f32) * 0.5) / (W as f32);
            let dy = (y as f32 - (H as f32) * 0.5) / (H as f32);
            let dist = (dx * dx + dy * dy).sqrt();

            let stripe = if ((x as f32 + y as f32 + index as f32 * 7.0) as i32 % 16) < 8 {
                1.0f32
            } else {
                0.85f32
            };

            let shade = (1.0 - dist * 1.2).clamp(0.2, 1.0) * stripe;

            let r = (base_r * shade * 255.0) as u8;
            let g = (base_g * shade * 255.0) as u8;
            let b = (base_b * shade * 255.0) as u8;

            slice[(y * W + x) as usize] = Rgba8Pixel { r, g, b, a: 255 };
        }
    }

    Image::from_rgba8(pixels)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h as i32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (r + m, g + m, b + m)
}

fn main() {
    let app = App::new().expect("Failed to create App window");

    // 使用 VecModel<ImageItem> 来动态更新 ListView。
    let model: Rc<VecModel<ImageItem>> = Rc::new(VecModel::default());
    app.set_items_model(ModelRc::from(model.clone()));

    // 记录已生成的条目数，作为"序列号"。
    let counter = Rc::new(std::cell::Cell::new(0u32));

    // "添加 100 条 mock 图片" 回调。
    {
        let model = model.clone();
        let counter = counter.clone();
        app.on_add_100_items(move || {
            let start = counter.get();
            for i in 0..100u32 {
                let idx = start + i;
                model.push(ImageItem {
                    image: make_mock_image(idx),
                    title: SharedString::from(format!("Mock Image #{}", idx + 1)),
                });
            }
            counter.set(start + 100);
        });
    }

    // "清空" 回调。
    {
        let model = model.clone();
        let counter = counter.clone();
        app.on_clear_items(move || {
            model.clear();
            counter.set(0);
        });
    }

    app.run().expect("Failed to run the event loop");
}
