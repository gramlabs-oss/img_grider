# ImgGrider

一个图像网格生成库。底层代码使用 Rust 编写，应用于 Elixir 生态。

## 使用

输入 9 张图片并提供生成方案，便可生成需要的单张九宫格（网格）图片。

```elixir
scheme = %ImgGrider.Scheme{
  target_dir: "_cache",
  indi_width: 180,
  indi_height: 120,
  watermark_font_family: "Lato"
}

photos = ["input/photo1.jpg", "input/photo2.jpg", ...]
```

调用：

```elixir
iex>ImgGrider.generate(photos, scheme)
{:ok, "_cache/8720b09b-075a-4c63-a326-e3cd277d0eb2.jpg"}
```

效果：

![Demo](./demo.jpg)
