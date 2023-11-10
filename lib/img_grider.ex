defmodule ImgGrider do
  use Rustler, otp_app: :img_grider, crate: "imggrider"

  # When your NIF is loaded, it will override this function.
  def generate(_photos), do: :erlang.nif_error(:nif_not_loaded)
end
