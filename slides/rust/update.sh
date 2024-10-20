set -e

compile_error=(
    "examples/arrays/numbers-change"
    "examples/booleans/other"
    "examples/errors/out-of-bounds-array"
    "examples/functions/declare-twice"
    "examples/intro/formatting-required"
    "examples/numbers/small-integers-unfit-in-i8"
    "examples/other/memory-leak"
    "examples/ownership/mutable-string-in-immutable-variable"
    "examples/rocket/calculator-with-get"
    "examples/sea-orm/counter"
    "examples/surrealdb/references-and-selects"
    "examples/variables/cannot-change-type"
    "examples/variables/change-literal-string"
    "examples/variables/immutable-number"
    "examples/variables/immutable-string"
)

tests_fail_intentionally=(
    "examples/macros/test-failure-report"
    "examples/rocket/return-result-user-id"
    "examples/rocket/skip-route"
    "examples/rocket/userid-in-path"
    "examples/testing/fixtures"
)

function all {
    for crate in examples/*/*
    do
        cd $root

        #echo $crate
        set +e
        is_dir=$(test -d $crate)
        if [ "$?" != "0" ]; then
            echo "SKIPPING $crate - this is a file not a folder."
            continue
        fi

        echo "======== update $crate"
        cd $crate
        set -e
        if [ "$crate" == "examples/threads/map-with-thread" ]; then
            echo "ERROR $crate - dependency is missing. The example probably has to be rewritten or removed"
            continue
        fi

        #cargo update
        #cargo fmt

        cargo_clippy

        cargo_test
        #cargo_run

    done
}

function cargo_clippy {
    for str in ${compile_error[@]}; do
        if [ "$crate" == "$str" ]; then
            echo "SKIPPING clippy $crate - this crate has an intentional compilation error"
            return
        fi
    done

    cargo clippy -- --deny warnings
}


function cargo_run {
    command_line=(
        "examples/args/args-demo"
        "examples/args/basic-math-operations"
        "examples/args/calc-args"
        "examples/json/read-json-from-reader-manually"
    )
    for str in ${command_line[@]}; do
        if [ "$crate" == "$str" ]; then
            echo "SKIPPING runnig $crate - command line options"
            continue 2
        fi
    done

    outfile=$(mktemp)
    cargo run > $outfile 2>&1
    compare to out.out
    if [ -e "out.out" ]; then
        diff out.out $outfile
    fi
}

function cargo_test {
    for str in ${compile_error[@]}; do
        if [ "$crate" == "$str" ]; then
            echo "SKIPPING clippy $crate - this crate has an intentional compilation error"
            return
        fi
    done

    for str in ${tests_fail_intentionally[@]}; do
        if [ "$crate" == "$str" ]; then
            echo "SKIPPING tests $crate - they fail intentionally"
            return
        fi
    done

    cargo test
}

root=$(pwd)
all

