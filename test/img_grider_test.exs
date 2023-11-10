defmodule ImgGriderTest do
  use ExUnit.Case
  doctest ImgGrider

  import ImgGrider

  test "generate/1" do
    assets_path = Path.join("test", "assets")

    photos =
      1..9
      |> Enum.map(fn n -> "photo-#{n}.jpg" end)
      |> Enum.map(fn name -> Path.join(assets_path, name) end)

    generate(photos)
  end
end
