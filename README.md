# goga [![Build Status](https://travis-ci.org/marioidival/goga.svg?branch=master)](https://travis-ci.org/marioidival/goga)
A fully RESTful API from any existing PostgreSQL database written in Rust

# Inspiration

This project is highly inspired on [pREST](https://github.com/nuveo/prest/). Then 4 years later, highly inspired from a conversation. Now it’s out of the grave


# TODO
- HTTP GET

	- [X] Get all DATABASES
	- [X] Get all SCHEMAS
	- [X] Get all TABLES
	- [X] SELECT `/db/sch/tbl?_select=column`
		- [X] select with all columns `*`
		- [X] select with specific column `_select=column1,column2`
	- [ ] WHERE `/db/sch/tbl?column=value`
		- [X] filter with operators `column=$gt.100`
		- [ ] filter with json/jsonb columns
	- [X] COUNT `/db/sch/tbl?_count=column`
		- [X] count with all columns `*`
		- [X] count with specific column `_count=column1,column2`
	- [X] ORDER BY `/db/sch/tbl?_order=column`
		- [X] order by with asc `_order=column` __default__
		- [X] order by with desc `_order=-column`
		- [X] order by with multiple orders `_order=-column1,column2`
	- [ ] GROUP BY `/db/sch/tbl?_select=column1,column2&_groupby=column1`
		- [X] group by chunk code with columns
		- [ ] group by with group functions support `SUM, AVG, MAX, MIN`
		- [ ] group by with having clause `_groupby=column1->>having:sum:column_name:$gt:500`
