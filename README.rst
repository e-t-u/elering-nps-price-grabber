Elering Nordpool Spot Price Data Grabber
========================================

Nordpool price grabber utilizing Elering Dashboard API.

Setup
-----

Create database schema:

.. code-block:: sql

   CREATE EXTENSION btree_gist;

   CREATE TABLE nordpool_price (
       "time" tsrange,
       "region" text,
       price decimal(8, 2),
       EXCLUDE USING GIST (region WITH =, time WITH &&)
   );

   CREATE INDEX ON nordpool_price (time, region);

Run the initial import (you need to run it at least twice),
and set up timer to fetch dataset once per day.

Known issues
------------

Upstream dataset contains few timestamps which cause exclusion
violations, for example data for `2018-10-28 01:00:00` is followed
by `2018-10-28 01:01:00` causing conflicts with current exclusion
rules due to range overlaps. As this is not a bookkeeping tool,
this issue is not properly handled and these records are ignored.

Erroneous records are following (probably caused by changes in
daylight savings time):
* 2018-10-28 01:01:00
* 2019-10-27 01:01:00
