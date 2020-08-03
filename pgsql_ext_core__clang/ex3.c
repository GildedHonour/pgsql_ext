#include "postgres.h"
#include "postgres_ext.h"
#include "executor/spi.h" /* this is what you need to work with SPI */
#include "commands/trigger.h" /* TriggerData */
#include "access/tupdesc.h"
#include "utils/rel.h"
#include "nodes/bitmapset.h"
#include "catalog/pg_attribute.h"
#include "fmgr.h"
#include "utils/builtins.h"
#include "utils/guc.h" // GetConfigOptionByName
#include "utils/elog.h"
#include "access/htup_details.h"
#include "access/sysattr.h"

// #include "settings.h"

#include <string.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <time.h>
#include <uuid/uuid.h>



extern Bitmapset *get_primary_key_attnos(Oid relid, bool deferrableOk, Oid *constraintOid);
extern int bms_num_members(const Bitmapset *a);
extern int bms_next_member(const Bitmapset *a, int prevbit);

            // extern PsqlSettings pset;

char* generate_uuid(void);
void write_data_to_file(void);

PG_MODULE_MAGIC;
extern Datum ex3_test(PG_FUNCTION_ARGS);
PG_FUNCTION_INFO_V1(ex3_test);
static int var1 = 1;

Datum ex3_test(PG_FUNCTION_ARGS) {
    TriggerData * trigdata = (TriggerData *) fcinfo->context;
    HeapTuple rettuple = trigdata->tg_trigtuple;

    if (!CALLED_AS_TRIGGER(fcinfo)) {
        elog(ERROR, "CALLED_AS_TRIGGER --> u no do it!!!");
    }

    if (TRIGGER_FIRED_BY_INSERT(trigdata->tg_event)) {
        TupleDesc tupdesc = trigdata->tg_relation->rd_att;


                        // if (fcinfo->pset != NULL) {
                        //     elog(INFO, "pset isn't NULL");
                        // }



        // name of table
        char *tbl = SPI_getrelname(trigdata->tg_relation);
        elog(INFO, "table: %s", tbl);
        char *schema = SPI_getnspname(trigdata->tg_relation);
        elog(INFO, "schema: %s", schema);


        bool isnull1, isnull2, isnull3, isnull4, isnull5, isnull5_2, isnull5_3 = false;
        uint32 x = rettuple->t_len;

        // config
        char *path1 = GetConfigOptionByName("my_ext.data_dir_path1", NULL, false);
        elog(INFO, "my_ext.data_dir_path1: %s", path1);

        //primary keys
        Oid constraintOid;
        Bitmapset *pkattnos = (Bitmapset*) get_primary_key_attnos(trigdata->tg_relation->rd_id, false, &constraintOid);
        if (pkattnos != NULL) {
            elog(INFO, "FirstLowInvalidHeapAttributeNumber: %d", FirstLowInvalidHeapAttributeNumber);
            int i = -1;
            while ((i = bms_next_member(pkattnos , i)) >= 0) {
                int col_idx = i + FirstLowInvalidHeapAttributeNumber;
                elog(INFO, "primary key col_idx: %d", col_idx);
            }
        } else {
            elog(INFO, "get_primary_key_attnos result NULL");
        }

     } else {
        elog(INFO, "non-insert trigger"); 
     }

    return PointerGetDatum(rettuple);
}

char* generate_uuid() {
    uuid_t bin_uuid;
    uuid_generate_random(bin_uuid);
    char *uuid = malloc(37);
    uuid_unparse_lower(bin_uuid, uuid);
    return uuid;
}

void write_data_to_file() {
    char *uuid1 = generate_uuid();
    char *path1 = "/Users/alex/projects/rust/pgsql__workspace/c_lang_examples/ex3/data_files/";
    char *file_ext = ".txt";
    char *file_full_path = (char *) malloc(strlen(uuid1) + strlen(path1) + strlen(file_ext) + 1);
    strcpy(file_full_path, path1);
    strcat(file_full_path, uuid1);
    strcat(file_full_path, file_ext);

    FILE *fp = fopen(file_full_path, "w");
    fprintf(fp, "test :%d", 123);
    fclose(fp);
}

//#define VARHDRSZ   ((int32) sizeof(int32))
//#define VARDATA_4B    (       PTR )      (((varattrib_4b *) (PTR))->va_4byte.va_data)
