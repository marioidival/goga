// Constant variables with statements of PostgreSQL
use std::fmt;

const FieldDatabaseName: &'static str = "datname";
const FieldSchemaName: &'static str = "schema_name";
const FieldCountDatabaseName: &'static str = format!("COUNT({})", FieldDatabaseName);
const FieldCountSchemaName: &'static str = format!("COUNT({})", FieldSchemaName);

// Database constants
const DatabaseSelect: &'static str = "SELECT {} FROM pg_database";
const DatabaseWhere: &'static str = " WHERE NOT datistemplate";
const DatabaseOrderBy: &'static str = " ORDER BY {} ASC"; // duplicated
const Databases: &'static str = format!(DatabaseSelect, FieldDatabaseName) + DatabaseWhere +
    format!(DatabaseOrderBy, FieldDatabaseName);

// Schemas constants
const SchemasSelect: &'static str = "SELECT {} FROM information_schema.schemata";
const SchemasGroupBy: &'static str = " GROUP BY {}";
const SchemasOrderBy: &'static str = " ORDER BY {} ASC"; // duplicated
const Schemas: &'static str = format!(SchemaSelect, FieldSchemaName) +
    format!(SchemaOrderBy, FieldSchemaName);

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
const Tables: &'static str = TablesSelect + TablesWhere + TablesOrderBy;

// Schemas + Tables constants
const SchemaTablesSelect: &'static str = "SELECT \
          t.tablename as \"name\" \
          t.schemaname as \"schema\", \
          sc.catalog_name as \"database\" \
          FROM pg_catalog.pg_tables t \
          INNER JOIN information_schema.schemata sc ON sc.schema_name = t.schemaname";
const SchemaTablesWhere: &'static str = " WHERE sc.catalog_name = $1 AND  t.schemaname = $2";
const SchemaTablesOrderBy: &'static str = " ORDER BY t.tablename ASC";
const SchemaTables: &'static str = SchemaTablesSelect + SchemaTablesWhere + SchemaTablesOrderBy;

// Some operations
const SelectAllInTable: &'static str = "SELECT * FROM";
const InsertQuery: &'static str = "INSERT INTO {}.{}.{}({}) VALUES({})";
const UpdateQuery: &'static str = "UPDATE {}.{}.{} SET {}";
const DeleteQuery: &'static str = "DELETE FROM {}.{}.{}";

const GroupBy: &'static str = "GROUP BY {}";
const Having: &'static str = "HAVING {} {} {}";
