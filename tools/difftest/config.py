"""Paths of the differential test (override with the environment)."""
import os
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.environ.get('BT3_GEN', os.path.join(HERE, '..', '..', 'game'))
REFDIR = os.environ.get('BT3_REF', os.path.join(HERE, 'build'))
REF = os.path.join(REFDIR, 'ref32')
TABLES = os.environ.get('BT3_TABLES', os.path.join(HERE, '..', '..', 'assets', 'data', 'runtime', 'source-tables.bt3'))
GAMEDATA = os.environ.get('BT3_DATA', os.path.join(HERE, '..', '..', 'assets', '_legacy', 'raw'))
WORK = os.environ.get('BT3_DIFF_WORK', os.path.join(REFDIR, 'work'))
try:  # machine-local overrides (not in the repository)
    from config_local import *  # noqa
    REF = os.path.join(REFDIR, 'ref32')
except ImportError:
    pass
