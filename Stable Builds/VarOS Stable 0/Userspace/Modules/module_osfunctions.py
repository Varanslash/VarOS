def statusreturn(code):
    with open("Userspace\\Transfer Pipelines\\statuscode.stat", "w") as f:
        f.write(code)
        exit()