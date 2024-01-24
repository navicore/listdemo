for tomorrow:

rework so that we only have one implementation of StatefulWidget in the List for
the root app.

then add Tables that will be custom to whatever context, ie: replicas or
services.

----------
learned that passing a collection of traits is best done passing a collection of
enumerations that have entries to pattern match for each type rather than use a
trait as superclass type.
