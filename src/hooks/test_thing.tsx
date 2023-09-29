async function testInvoke_try_compile_code() {
    // raise an exception with CompilationError[]

    let errArr: CompilationError[] = [
      {
        line_number: 0,
            column_number: 0,
        length: 5,
        message: "some message",
        suggestions: ["some suggestion", "some other suggestion"],
      },
    ];

    setTimeout(() => {
        throw errArr;
    }
    , 200);

}