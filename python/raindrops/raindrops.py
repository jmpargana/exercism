def convert(number):
    result = "Pling" if number % 3 == 0else ""

    if number % 5 == 0:
        result += "Plang"

    if number % 7 == 0:
        result += "Plong"

    return str(number) if not result else result
