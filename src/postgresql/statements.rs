// Constant variables with statements of PostgreSQL

const FIELD_DATABASE_NAME: &'static str = "datname";
const FIELD_SCHEMA_NAME: &'static str = "schema_name";
const DATABASE_WHERE: &'static str = " WHERE NOT datistemplate";

// TABLES constants
const TABLES_SELECT: &'static str =
    "SELECT \
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

const TABLES_WHERE: &'static str = " WHERE \
                                    c.relkind IN ('r','v','m','S','s','') AND \
                                    n.nspname !~ '^pg_toast' AND \
                                    n.nspname NOT IN ('information_schema', 'pg_catalog') AND \
                                    has_schema_privilege(n.nspname, 'USAGE')";
const TABLES_ORDERBY: &'static str = " ORDER BY 1, 2";

// SCHEMAS + TABLES constants
const SCHEMA_TABLES_SELECT: &'static str = "SELECT
          t.tablename as \"name\",
          t.schemaname as \"schema\",
          sc.catalog_name as \"database\"
          FROM pg_catalog.pg_tables t
          INNER JOIN information_schema.schemata sc ON sc.schema_name = t.schemaname";
const SCHEMA_TABLES_WHERE: &'static str = "WHERE sc.catalog_name = $1 AND t.schemaname = $2";
const SCHEMA_TABLES_ORDERBY: &'static str = "ORDER BY t.tablename ASC";

lazy_static! {

    static ref FIELD_COUNT_DATABASE_NAME: String = format!("COUNT({})", FIELD_DATABASE_NAME);
    static ref FIELD_COUNT_SCHEMA_NAME: String = format!("COUNT({})", FIELD_SCHEMA_NAME);

    // Database constants
    static ref DATABASE_SELECT: String = String::from("SELECT {} FROM pg_database");
    static ref DATABASE_ORDERBY: String = String::from(" ORDER BY {} ASC"); // duplicated
    pub static ref DATABASES: String = format!(
        "{}", DATABASE_SELECT.replace("{}", FIELD_DATABASE_NAME)
    ) + DATABASE_WHERE + &format!("{}", DATABASE_ORDERBY.replace("{}", FIELD_DATABASE_NAME));

    // SCHEMAS constants
    static ref SCHEMAS_SELECT: String = String::from("SELECT {} FROM information_schema.schemata");
    static ref SCHEMAS_GROUPBY: String = String::from(" GROUP BY {}");
    static ref SCHEMAS_ORDERBY: String = String::from(" ORDER BY {} ASC"); // duplicated
    pub static ref SCHEMAS: String = format!(
        "{}", SCHEMAS_SELECT.replace("{}", FIELD_SCHEMA_NAME)
    ) + &format!("{}", SCHEMAS_ORDERBY.replace("{}", FIELD_SCHEMA_NAME));


    pub static ref TABLES: String = format!(
        "{} {} {}", TABLES_SELECT, TABLES_WHERE, TABLES_ORDERBY
    );

    pub static ref SCHEMA_TABLES: String = format!(
        "{} {} {}",
        SCHEMA_TABLES_SELECT,
        SCHEMA_TABLES_WHERE,
        SCHEMA_TABLES_ORDERBY
    );

    // Some operations
    static ref SELECT_ALL_IN_TABLE: &'static str = "SELECT * FROM";
    static ref INSERT_QUERY: String = String::from("INSERT INTO {}.{}.{}({}) VALUES({})");
    static ref UPDATE_QUERY: String = String::from("UPDATE {}.{}.{} SET {}");
    static ref DELETE_QUERY: String = String::from("DELETE FROM {}.{}.{}");

    static ref GROUPBY: String = String::from(" GROUP BY {}");
    static ref HAVING: String = String::from(" HAVING {} {} {}");
}
