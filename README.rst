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
