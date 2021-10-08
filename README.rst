WIP!
====

Nordpool price grabber utilizing Elering Dashboard API.

Setup
-----

Create database schema:

.. code-block:: sql

    CREATE TABLE nordpool_price (
        "time" tsrange,
        "region" text,
        price decimal(8, 2)
    );

    CREATE INDEX ON nordpool_price (time, region);
