defmodule ImgGrider.Scheme do
  @moduledoc false

  @enforce_keys [
    :target_dir,
    :indi_width,
    :indi_height
  ]

  defstruct target_dir: nil,
            format: "jpg",
            indi_width: nil,
            indi_height: nil,
            watermark_font_family: "FreeMono",
            watermark_font_size: 54.0,
            watermark_font_weight: 600

  @type t :: %__MODULE__{
          target_dir: String.t(),
          format: String.t(),
          indi_width: non_neg_integer(),
          indi_height: non_neg_integer(),
          watermark_font_family: String.t(),
          watermark_font_size: float(),
          watermark_font_weight: non_neg_integer()
        }
end
