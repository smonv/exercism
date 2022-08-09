defmodule BoutiqueInventory do
  @type item() :: %{
          name: String.t(),
          price: non_neg_integer(),
          quantity_by_size: %{:atom => non_neg_integer()}
        }

  @type inventory() :: [item()]

  @spec sort_by_price(inventory()) :: [inventory()]
  def sort_by_price(inventory) do
    inventory |> Enum.sort_by(& &1.price)
  end

  @spec with_missing_price(inventory) :: [inventory()]
  def with_missing_price(inventory) do
    inventory |> Enum.filter(&is_nil(&1.price))
  end

  def update_names(inventory, old_word, new_word) do
    inventory
    |> Enum.map(fn item ->
      Map.put(item, :name, item.name |> String.replace(old_word, new_word))
    end)
  end

  # @spec increase_quantity(item()) :: item()
  def increase_quantity(item, count) do
    Map.put(
      item,
      :quantity_by_size,
      item.quantity_by_size
      |> Map.new(fn {k, v} -> {k, v + count} end)
      |> Enum.into(%{})
    )
  end

  @spec total_quantity(item()) :: non_neg_integer()
  def total_quantity(item) do
    item.quantity_by_size |> Map.values() |> Enum.reduce(0, &+/2)
  end
end
