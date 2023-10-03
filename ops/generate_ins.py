import pyperclip

def generate_ins():
    ins_arr: list[str] = []
    for i in range(0x100):
        # ins_str = ins_str + f"0x8b, {hex(i)}, "
        ins_arr.append(f"{hex(i)}")

    return ins_arr

def append_ins(ins: list[str], prefix: int) -> list[str]:
    return [f"{hex(prefix)}, {i}" for i in ins]

def to_var_in_each_line(ins: list[str]) -> list[str]:
    return [f"var{i} db {val}" for i, val in enumerate(ins)]

def to_str(ins: list[str]) -> str:
    return ", ".join(ins)

def to_str_with_newline(ins: list[str]) -> str:
    return "\n".join(ins)

def seperate(ins: list[str]) -> list[str]:
    # for every 3f elements, sperate them with a newline
    sperated: list[str] = []
    for i, val in enumerate(ins):
        if i % 0x40 == 0:
            sperated.append("; ------------------")
        sperated.append(val)
    return sperated


pyperclip.copy( to_str_with_newline( 
    seperate(
    to_var_in_each_line( 
        append_ins(
            generate_ins(), 0x03) ) )))

