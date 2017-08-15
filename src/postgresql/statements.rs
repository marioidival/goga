// Constant variables with statements of PostgreSQL
use std::fmt;

const FieldDatabaseName: &'static str = "datname";
const FieldSchemaName: &'static str = "schema_name";
const DatabaseWhere: &'static str = " WHERE NOT datistemplate";

// Tables constants
const TablesSelect: &'static str = "SELECT \
n.nspname as \"schema\", c.relname as \"name\", \
CASE c.relkind \
WHEN 'r' THEN 'table' \
WHEN 'v' THEN 'view' \
WHEN 'm' THEN 'materialized_view' \
WHEN 'i' THEN 'index' \
WHEN 'S' THEN 'sequence' \
WHEN 's' THEN 'special' \
WHEN 'f' THEN 'foreign_table' \
END as \"type\", \
pg_catalog.pg_get_userbyid(c.relowner) as \"owner\" \
FROM pg_catalog.pg_class c LEFT JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace";

const TablesWhere: &'static str = " WHERE \
c.relkind IN ('r','v','m','S','s','') AND \
n.nspname !~ '^pg_toast' AND \
n.nspname NOT IN ('information_schema', 'pg_catalog') AND \
has_schema_privilege(n.nspname, 'USAGE')";
const TablesOrderBy: &'static str = " ORDER BY 1, 2";

// Schemas + Tables constants
const SchemaTablesSelect: &'static str = "SELECT
          t.tablename as \"name\",
          t.schemaname as \"schema\",
          sc.catalog_name as \"database\" 
          FROM pg_catalog.pg_tables t
          INNER JOIN information_schema.schemata sc ON sc.schema_name = t.schemaname";
const SchemaTablesWhere: &'static str = "WHERE sc.catalog_name = $1 AND t.schemaname = $2";
const SchemaTablesOrderBy: &'static str = "ORDER BY t.tablename ASC";



lazy_static! {

    static ref FieldCountDatabaseName: String = format!("COUNT({})", FieldDatabaseName);
    static ref FieldCountSchemaName: String = format!("COUNT({})", FieldSchemaName);

    // Database constants
    static ref DatabaseSelect: String = String::from("SELECT {} FROM pg_database");
    static ref DatabaseOrderBy: String = String::from(" ORDER BY {} ASC"); // duplicated
    pub static ref Databases: String = format!("{}", DatabaseSelect.replace("{}", FieldDatabaseName)) + DatabaseWhere +
        &format!("{}", DatabaseOrderBy.replace("{}", FieldDatabaseName));

    // Schemas constants
    static ref SchemasSelect: String = String::from("SELECT {} FROM information_schema.schemata");
    static ref SchemasGroupBy: String = String::from(" GROUP BY {}");
    static ref SchemasOrderBy: String = String::from(" ORDER BY {} ASC"); // duplicated
    pub static ref Schemas: String = format!("{}", SchemasSelect.replace("{}", FieldSchemaName)) +
        &format!("{}", SchemasOrderBy.replace("{}", FieldSchemaName));


    pub static ref Tables: String = format!("{} {} {}", TablesSelect, TablesWhere, TablesOrderBy);

    pub static ref SchemaTables: String = format!("{} {} {}", SchemaTablesSelect, SchemaTablesWhere, SchemaTablesOrderBy);

    // Some operations
    static ref SelectAllInTable: &'static str = "SELECT * FROM";
    static ref InsertQuery: String = String::from("INSERT INTO {}.{}.{}({}) VALUES({})");
    static ref UpdateQuery: String = String::from("UPDATE {}.{}.{} SET {}");
    static ref DeleteQuery: String = String::from("DELETE FROM {}.{}.{}");

    static ref GroupBy: String = String::from(" GROUP BY {}");
    static ref Having: String = String::from(" HAVING {} {} {}");
}
