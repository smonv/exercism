defmodule LibraryFees do
  def datetime_from_string(string) do
    NaiveDateTime.from_iso8601!(string)
  end

  def before_noon?(datetime) do
    datetime.hour < 12
  end

  def return_date(checkout_datetime) do
    hour = checkout_datetime.hour
    days = if hour >= 12, do: 29, else: 28
    checkout_datetime |> NaiveDateTime.to_date() |> Date.add(days)
  end

  def days_late(planned_return_date, actual_return_datetime) do
    Date.diff(return_date(actual_return_datetime), planned_return_date)
  end

  def monday?(datetime) do
    datetime.calendar().day_of_week() == 0
    # Please implement the monday?/1 function
  end

  def calculate_late_fee(checkout, return, rate) do
    # Please implement the calculate_late_fee/3 function
  end
end
