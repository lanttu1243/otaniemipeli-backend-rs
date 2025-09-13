with open(r"../src/utils/types.rs", "r") as f:
    lines = f.readlines()

ts_lines = []
in_type: bool = False
for line in lines:
    if ("#[" in line
            or line.startswith("use ")
            or line.startswith("impl ")
            or line.strip() == ""
            or "pub type" in line):
        continue
    ts_lines.append(line
                    .replace("pub struct", "export interface")
                    .replace("pub enum", "export type")
                    .replace("\n", "")
                    .replace("String", "string")
                    .replace("i32", "number")
                    .replace("f64", "number")
                    .replace("bool", "boolean")
                    .replace("DateTime<Utc>", "string")
                    .replace("Vec<", "")
                    .replace(">", "[]")
                    .replace("pub ", ""))

with open(r"./types.ts", "w+") as f:
    for line in ts_lines:
        f.write(line + "\n")
