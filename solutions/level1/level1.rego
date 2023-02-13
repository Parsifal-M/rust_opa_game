# Do not edit the package declaration
package barista

# Create the rule below
deny[message] {
    input.Order.Drink == "Cola"
    message := sprintf("Unfortunately, we do not serve %s", [input.Order.Drink])
}