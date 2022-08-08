defmodule KitchenCalculator do
  def get_volume(volume_pair) do
    {_, volume} = volume_pair
    volume
  end

  def to_milliliter(volume_pair) do
    {unit, volume} = volume_pair

    ml_volume =
      case unit do
        :cup -> 240 * volume
        :fluid_ounce -> 30 * volume
        :teaspoon -> 5 * volume
        :tablespoon -> 15 * volume
        :milliliter -> volume
      end

    {:milliliter, ml_volume}
  end

  def from_milliliter(volume_pair, unit) do
    {_, volume} = volume_pair

    v =
      case unit do
        :cup -> volume / 240
        :fluid_ounce -> volume / 30
        :teaspoon -> volume / 5
        :tablespoon -> volume / 15
        :milliliter -> volume
      end

    {unit, v}
  end

  def convert(volume_pair, unit) do
    {input_unit, volume} = volume_pair

    case input_unit do
      ^unit -> volume
      :milliliter -> from_milliliter(volume_pair, unit)
      _ -> to_milliliter(volume_pair) |> from_milliliter(unit)
    end
  end
end
