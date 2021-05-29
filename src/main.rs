mod repository;

fn main() {
    let currencies_repository = repository::Repository {
        url: "https://github.com/datasets/currency-codes",
        path: "./resources/currency-codes",
        branch: "master",
    };

    currencies_repository.check();
    //currencies_repository.pull();
    print!("data has been processed")
}
