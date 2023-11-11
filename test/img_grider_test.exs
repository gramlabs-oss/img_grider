defmodule ImgGriderTest do
  use ExUnit.Case
  doctest ImgGrider

  import ImgGrider

  alias ImgGrider.Scheme

  test "generate/1" do
    assets_path = Path.join("test", "assets")

    scheme = %Scheme{
      target_dir: Path.join(assets_path, "output"),
      indi_width: 180,
      indi_height: 120
    }

    photos =
      1..9
      |> Enum.map(fn n -> "photo-#{n}.jpg" end)
      |> Enum.map(fn name -> Path.join(assets_path, name) end)

    generate(photos, scheme)
  end
end
