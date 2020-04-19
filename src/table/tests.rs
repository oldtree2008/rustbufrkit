use super::*;
use crate::table::table::{TableGroup, TableGroupId, TableGroupManager};
use std::ptr;
use std::ops::Deref;
use crate::table::template::{expand_one, Node, Template, PrintVisitor};
use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::borrow::Borrow;

#[test]
fn test_table_group_manager() {
    let mut tgm = TableGroupManager::new();
    let tg_id = TableGroupId {
        base_dir: String::from("_definitions/tables"),
        master_table_number: 0,
        centre_number: 0,
        sub_centre_number: 0,
        version_number: 25,
    };
    let t1 = tgm.get_table_group(&tg_id).unwrap();
    let t2 = tgm.get_table_group(&tg_id).unwrap();
    assert_eq!(1, tgm.size());
    ptr::eq(t1.deref(), t2.deref());
}

#[test]
fn test_load_table_group() {
    let table_group = create_table_group();

    assert_eq!("id=_definitions/tables/0/0_0/25", format!("{}", table_group))
}

#[test]
fn test_lookup_descriptor() {
    let table_group = create_table_group();

    table_group.lookup(1001).unwrap();
    table_group.lookup(101000).unwrap();
    table_group.lookup(201011).unwrap();
    table_group.lookup(225255).unwrap();
    table_group.lookup(300002).unwrap();
}

#[test]
#[should_panic]
fn test_lookup_bad_descriptor() {
    let table_group = create_table_group();
    table_group.lookup(987654).unwrap();
}

#[test]
fn test_lookup_cnf() {
    let table_group = create_table_group();

    assert_eq!("REGION V", table_group.lookup_cnf(1003, 5).unwrap());
    assert_eq!("REGION V", table_group.lookup_cnf(1003, 5).unwrap());
}

#[test]
fn test_lookup_meta() {
    let table_group = create_table_group();

    assert_eq!(
        "Identification: Identifies origin and type of data",
        table_group.lookup_meta(1001).unwrap(),
    );

    assert_eq!(
        "Change data width",
        table_group.lookup_meta(201011).unwrap(),
    );

    assert_eq!(
        "Difference statistical values follow",
        table_group.lookup_meta(225000).unwrap(),
    );

    assert_eq!(
        "Location and identification sequences",
        table_group.lookup_meta(301059).unwrap(),
    );
}

#[test]
fn test_data_category_of() {
    let table_group = create_table_group();

    assert_eq!(
        "Single level upper - air data (satellite)",
        table_group.data_category_of(5).unwrap(),
    );

    assert_eq!(
        "Reserved",
        table_group.data_category_of(100).unwrap(),
    );
}

#[test]
fn test_template() {
    let table_group = create_table_group();
    let template = Template::new(
        &table_group, &[302059, 1001]).unwrap();
    println!("Template is {:?}", template)
}

#[test]
fn test_print_visitor() {
    let table_group = create_table_group();
    let template = Template::new(
        &table_group, &[309052]).unwrap();

    let mut print_visitor = PrintVisitor::new();
    template.accept(&mut print_visitor);
}

fn create_table_group() -> TableGroup {
    TableGroup::load(&TableGroupId {
        base_dir: String::from("_definitions/tables"),
        master_table_number: 0,
        centre_number: 0,
        sub_centre_number: 0,
        version_number: 25,
    }).unwrap()
}