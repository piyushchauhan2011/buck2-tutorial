# def assert_output(name, command, output):
#     return native.genrule(
#         name = name,
#         bash = f"""set -e
# {command} | grep -Fx "{output}"
# touch "$OUT"
# """,
#         cmd_exe = f"""{command} | findstr \""{output}" > nul && type nul > "$OUT" """,
#         out = "out.txt",
#     )

def assert_output(name, command, output):
    return native.genrule(
        name = name,
        bash = command + " | grep \"" + output + "\" && touch \"$OUT\"",
        cmd_exe = command + " | findstr \"" + output + "\" && type nul > \"$OUT\"",
        out = "out.txt",
    )
