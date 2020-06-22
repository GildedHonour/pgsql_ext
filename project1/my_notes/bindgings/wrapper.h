#include "postgres.h"
// include "postgres_ext.h"
#include "executor/spi.h"       /* this is what you need to work with SPI */
#include "commands/trigger.h"   /* TriggerData    */
#include "access/tupdesc.h"
#include "utils/rel.h"
#include "nodes/bitmapset.h"
#include "catalog/pg_attribute.h"

#include "fmgr.h"
#include "utils/builtins.h"
#include "utils/guc.h" // GetConfigOptionByName
// include "utils/elog.h"

