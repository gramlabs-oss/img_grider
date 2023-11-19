defmodule ImgGrider do
  @moduledoc false

  use Rustler, otp_app: :img_grider, crate: "imggrider"

  alias ImgGrider.{Scheme, Error}

  # When your NIF is loaded, it will override this function.
  @spec generate([String.t()], Scheme.t()) :: {:ok, String.t()} | {:error, Error.t()}
  def generate(_photos, _scheme), do: :erlang.nif_error(:nif_not_loaded)
end
