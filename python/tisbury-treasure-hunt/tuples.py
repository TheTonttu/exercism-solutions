"""Functions to help Azara and Rui locate pirate treasure."""

from __future__ import annotations
from typing import NamedTuple

Coordinate = str
Treasure = str
CoordinateX = str
CoordinateY = str
Quadrant = str
Location = str
TreasureRecord = tuple[Treasure, Coordinate]
CoordinateComponents = tuple[CoordinateX, CoordinateY]
LocationRecord = tuple[Location, CoordinateComponents, Quadrant]
CombinedRecord = tuple[Treasure, Coordinate, Location, CoordinateComponents, Quadrant]
FormattedRecords = str
NoMatch = str

NO_MATCH = "not a match"


def get_coordinate(record: TreasureRecord) -> Coordinate:
    """Return coordinate value from a tuple containing the treasure name, and treasure coordinate.

    :param record: tuple - with a (treasure, coordinate) pair.
    :return: str - the extracted map coordinate.
    """
    return __TreasureRecord(*record).coordinate


def convert_coordinate(coordinate: Coordinate) -> CoordinateComponents:
    """Split the given coordinate into tuple containing its individual components.

    :param coordinate: str - a string map coordinate
    :return: tuple - the string coordinate split into its individual components.
    """
    return __CoordinateComponents(coordinate[0], coordinate[-1])


def compare_records(
    azara_record: TreasureRecord,
    rui_record: LocationRecord,
) -> bool:
    """Compare two record types and determine if their coordinates match.

    :param azara_record: tuple - a (treasure, coordinate) pair.
    :param rui_record: tuple - a (location, tuple(coordinate_1, coordinate_2), quadrant) trio.
    :return: bool - do the coordinates match?
    """
    record = __TreasureRecord(*azara_record)
    quadrant_record = __LocationRecord(*rui_record)
    return (
        convert_coordinate(record.coordinate) == quadrant_record.coordinate_components
    )


def create_record(
    azara_record: TreasureRecord,
    rui_record: LocationRecord,
) -> CombinedRecord | NoMatch:
    """Combine the two record types (if possible) and create a combined record group.

    :param azara_record: tuple - a (treasure, coordinate) pair.
    :param rui_record: tuple - a (location, coordinate, quadrant) trio.
    :return: tuple or str - the combined record (if compatible), or the string "not a match" (if incompatible).
    """
    if compare_records(azara_record, rui_record):
        return __CombinedRecord(*azara_record, *rui_record)

    return NO_MATCH


def clean_up(combined_record_group: tuple[CombinedRecord, ...]) -> FormattedRecords:
    """Clean up a combined record group into a multi-line string of single records.

    :param combined_record_group: tuple - everything from both participants.
    :return: str - everything "cleaned", excess coordinates and information are removed.

    The return statement should be a multi-lined string with items separated by newlines.

    (see HINTS.md for an example).
    """

    return (
        "\n".join((str((record[0], *record[2:])) for record in combined_record_group))
        + "\n"
    )


class __TreasureRecord(NamedTuple):
    treasure: Treasure
    coordinate: Coordinate


class __CoordinateComponents(NamedTuple):
    x: CoordinateX
    y: CoordinateY


class __LocationRecord(NamedTuple):
    location: Location
    coordinate_components: CoordinateComponents
    quadrant: Quadrant


class __CombinedRecord(NamedTuple):
    tresure: Treasure
    coordinate: Coordinate
    location: Location
    coordinate_components: CoordinateComponents
    quadrant: Quadrant
