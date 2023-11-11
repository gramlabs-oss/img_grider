assets_path = Path.join("test", "assets")

scheme = %ImgGrider.Scheme{
  target_dir: Path.join(assets_path, "output"),
  indi_width: 180,
  indi_height: 120
}

photos =
  1..9
  |> Enum.map(fn n -> "photo-#{n}.jpg" end)
  |> Enum.map(fn name -> Path.join(assets_path, name) end)

Benchee.run(%{
  "ImgGrider.generate/2" => fn -> ImgGrider.generate(photos, scheme) end
})
