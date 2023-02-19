# Do not edit the package declaration
package barista

# Create the rule below

deny[message] {
    input.Member.Premium == "false"
    message := "Unfortunately, you are not a premium member, and cannot have an extra shot of espresso"
}