defmodule ImgGriderTest do
  use ExUnit.Case
  doctest ImgGrider

  import ImgGrider

  alias ImgGrider.Scheme

  @assets_path Path.join("test", "assets")
  @scheme %Scheme{
    target_dir: Path.join(@assets_path, "output"),
    indi_width: 180,
    indi_height: 120
  }

  test "generate/2" do
    photos =
      1..9
      |> Enum.map(fn n -> "photo-#{n}.jpg" end)
      |> Enum.map(fn name -> Path.join(@assets_path, name) end)

    {:ok, path} = generate(photos, @scheme)

    assert File.exists?(path)
  end

  test "generate_error_result" do
    {:error, error} = generate(["not_found.jpg"], @scheme)

    assert is_struct(error, ImgGrider.Error)
    assert error.kind == :magick_exception
    assert error.message == "failed to read image"
  end
end
