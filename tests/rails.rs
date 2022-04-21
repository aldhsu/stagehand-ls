use stagehand_lsp::projection::ProjectionCollection;

#[test]
fn it_can_load_the_fixture() {
    let file = std::fs::read_to_string("tests/fixtures/rails_default.json").unwrap();
    let projections = ProjectionCollection::new(&file).unwrap();
}
