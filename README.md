# goga [![Build Status](https://travis-ci.org/marioidival/goga.svg?branch=master)](https://travis-ci.org/marioidival/goga)
A fully RESTful API from any existing PostgreSQL database written in Rust

# Inspiration

This project is highly inspired on [pREST](https://github.com/nuveo/prest/)


# TODO
- HTTP GET
	- [ ] SELECT `/db/sch/tbl?_select=column`
		- [ ] select with all columns `*`
		- [ ] select with specific column `_select=column1,column2`
	- [ ] WHERE `/db/sch/tbl?column=value`
		- [X] filter with operators `column=$gt.100`
		- [ ] filter with json/jsonb columns
	- [ ] COUNT `/db/sch/tbl?_count=column`
		- [ ] count with all columns `*`
		- [ ] count with specific column `_count=column1,column2`
	- [ ] ORDER BY `/db/sch/tbl?_order=column`
		- [ ] order by with asc `_order=column` __default__
		- [ ] order by with desc `_order=-column`
		- [ ] order by with multiple orders `_order=-column1,column2`
	- GROUP BY `/db/sch/tbl?_select=column1,column2&_groupby=column1`
		- [ ] group by with group functions support `SUM, AVG, MAX, MIN`
		- [ ] group by with having clause `_groupby=column1->>having:sum:column_name:$gt:500`
