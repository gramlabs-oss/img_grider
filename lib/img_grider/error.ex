defmodule ImgGrider.Error do
  @moduledoc false

  defstruct [:kind, :message]

  @type kind :: :magick_exception | :other

  @type t :: %__MODULE__{
          kind: kind(),
          message: String.t()
        }
end
