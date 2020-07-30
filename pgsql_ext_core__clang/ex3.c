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

char* generate_uuid(void);
void write_data_to_file(void);

PG_MODULE_MAGIC;
extern Datum ex3_test(PG_FUNCTION_ARGS);
PG_FUNCTION_INFO_V1(ex3_test);
static int var1 = 1;

Datum ex3_test(PG_FUNCTION_ARGS) {
    ++var1;
    elog(INFO, "static var1: %d", var1);

    TriggerData * trigdata = (TriggerData *) fcinfo->context;
    HeapTuple rettuple = trigdata->tg_trigtuple;

    if (!CALLED_AS_TRIGGER(fcinfo)) {
        elog(ERROR, "CALLED_AS_TRIGGER --> u no do it!!!");
    }

    if (TRIGGER_FIRED_BY_INSERT(trigdata->tg_event)) {
        TupleDesc tupdesc = trigdata->tg_relation->rd_att;
        bool isnull1, isnull2, isnull3, isnull4, isnull5, isnull5_2, isnull5_3 = false;
        uint32 x = rettuple->t_len;

        // config
        char *path1 = GetConfigOptionByName("my_ext.data_dir_path1", NULL, false);
        elog(INFO, "my_ext.data_dir_path1: %s", path1);








        // //indexing begins at 1
        // int32 att1 = DatumGetInt32(heap_getattr(rettuple, 1, tupdesc, &isnull1));
        // int32 att2 = DatumGetInt32(heap_getattr(rettuple, 2, tupdesc, &isnull2));
        // char *att3 = DatumGetCString(heap_getattr(rettuple, 3, tupdesc, &isnull3));
        // // char *att4 = DatumGetCString(heap_getattr(rettuple, 4, tupdesc, &isnull4));
        // // void *att5 = DatumGetCString(heap_getattr(rettuple, 5, tupdesc, &isnull5));

        // /*
        //     If you have not looked at DatumGetVarBitP() and DatumGetByteaP(), that
        //     will get you corresponding structure pointers from a Datum. Then check
        //     src/backend/utils/adt/varbit.c and bytea_* functions from
        //     src/backend/utils/adt/varlena.c to understand how those structures are
        //     used.
        // */

        // // void *a1 = (void *) DatumGetByteaP(heap_getattr(rettuple, 5, tupdesc, &isnull5));
        // // bytea *a2 = DatumGetByteaP(heap_getattr(rettuple, 5, tupdesc, &isnull5));

        // // char * SPI_getvalue(HeapTuple row, TupleDesc rowdesc, int colnumber)
        // // Datum SPI_getbinval(HeapTuple row, TupleDesc rowdesc, int colnumber, bool * isnull)

        // void *att5 = (void *) DatumGetByteaP(heap_getattr(rettuple, 5, tupdesc, &isnull5));
        // bytea *att5_2 = DatumGetByteaP(heap_getattr(rettuple, 5, tupdesc, &isnull5_2));
        // bytea *att5_3 = SPI_getbinval(rettuple, tupdesc, 5, &isnull5_3);


        // if (!isnull1) {
        //     elog(INFO, "insert tigger: data[1]: %d", att1);
        // }
        // else {
        //     elog(INFO, "insert tigger: data[1]: NULL");
        // }

        // if (!isnull2) {
        //     elog(INFO, "insert tigger: data[2]: %d", att2);
        // }
        // else {
        //     elog(INFO, "insert tigger: data[2]: NULL");
        // }

        // if (!isnull3) {
        //     elog(INFO, "insert tigger: data[3]: %s", att3);
        // }
        // else {
        //     elog(INFO, "insert tigger: data[3]: NULL");
        // }


        // // if (!isnull4) {
        // //     elog(INFO, "insert tigger: data[4]: %s", att4);
        // // }
        // // else {
        // //     elog(INFO, "insert tigger: data[4]: NULL");
        // // }


        // if (!isnull5) {
        //     elog(INFO, "insert tigger: data[5]: %u", att5);
        //     elog(INFO, "sizeof: data[5]: %d", sizeof(att5));
        //     elog(INFO, "VARSIZE: data[5]: %u", VARSIZE(att5));

        //     elog(INFO, "sizeof: data[5_2]: %d", sizeof(att5_2));
        //     elog(INFO, "VARSIZE: data[5_2]: %u", VARSIZE(att5_2));

        //     elog(INFO, "sizeof: data[5_3]: %d", sizeof(att5_3));
        //     elog(INFO, "VARSIZE: data[5_3]: %u", VARSIZE(att5_3));


        //     elog(INFO, "VARSIZE_ANY_EXHDR: data[5_3]: %u", VARSIZE_ANY_EXHDR(att5_3));
        //     elog(INFO, "VARHDRSZ: %u", VARHDRSZ);
        //     elog(INFO, "VARHDRSZ: %u", VARHDRSZ);


        //     char *uuid1 = generate_uuid();
        //     char *path1 = "/Users/alex/projects/rust/pgsql__workspace/c_lang_examples/ex3/data_files/";
        //     char *file_ext = ".jpg";
        //     // char *file_ext = ".png";
        //     char *file_full_path = (char *) malloc(strlen(uuid1) + strlen(path1) + strlen(file_ext) + 1);
        //     strcpy(file_full_path, path1);
        //     strcat(file_full_path, uuid1);
        //     strcat(file_full_path, file_ext);

        //     FILE *fp = fopen(file_full_path, "w");
        //     fwrite((att5 + VARHDRSZ), 1, VARDATA(att5), fp);
        //     // fwrite((att5 + VARHDRSZ), 4, VARDATA(att5), fp);
        //     // fwrite((att5 + VARHDRSZ), 8, VARDATA(att5), fp);

        //     fclose(fp);


        // }
        // else {
        //     elog(INFO, "insert tigger: data[5]: NULL");
        // }

        // //debug
        // elog(INFO, "-------------\r\n");
        // elog(INFO, "tupdesc->natts: %d", tupdesc->natts);
        // elog(INFO, "tupdesc->tdtypeid: %u", tupdesc->tdtypeid);
        // elog(INFO, "tupdesc->tdtypmod: %d", tupdesc->tdtypmod);
        // elog(INFO, "tupdesc->tdrefcount: %d", tupdesc->tdrefcount);
        // elog(INFO, "-------------\r\n");

        // //column names
        // elog(INFO, "tupdesc->attrs[0] attrelid: %x", tupdesc->attrs[0].attrelid);
        // elog(INFO, "tupdesc->attrs[0] attname data: %s", tupdesc->attrs[0].attname.data);
        // elog(INFO, "tupdesc->attrs[1] attname data: %s", tupdesc->attrs[1].attname.data);
        // elog(INFO, "tupdesc->attrs[2] attname data: %s", tupdesc->attrs[2].attname.data);
        // elog(INFO, "tupdesc->attrs[3] attname data: %s", tupdesc->attrs[3].attname.data);
        // elog(INFO, "tupdesc->attrs[4] attname data: %s", tupdesc->attrs[4].attname.data);
        // elog(INFO, "-------------\r\n");



        // // select conrelid from pg_catalog.pg_constraint;
        // // 57349
        // elog(INFO, "trigdata->tg_relation->rd_id: %d", trigdata->tg_relation->rd_id);

        // //select * from pg_catalog.pg_class where relname = 'table1';
        // elog(INFO, "trigdata->tg_relation->rd_rel->relname.data: %s", trigdata->tg_relation->rd_rel->relname.data);

        // // select conindid from pg_catalog.pg_constraint;
        // // 57357
        // elog(INFO, "trigdata->tg_relation->rd_pkindex: %d", trigdata->tg_relation->rd_pkindex);
        // elog(INFO, "-------------\r\n");


        // for (int i = 0; i < 10; i++) {
        //     char *t = SPI_gettype(tupdesc, i);
        //     elog(INFO, "SPI_gettype of tupdesc, col[%d]: %s", i, t);
        // }
        // elog(INFO, "-------------\r\n");










        //primary keys
        // 
        Oid constraintOid;
        Bitmapset *pkattnos = (Bitmapset*) get_primary_key_attnos(trigdata->tg_relation->rd_id, false, &constraintOid);
        if (pkattnos != NULL) {
            /*
            elog(INFO, "bms_num_members: %d", bms_num_members(res1));
            elog(INFO, "bms_next_member: %d", bms_next_member(res1, 0));
            elog(INFO, "nwords: %d", res1->nwords);
            elog(INFO, "words [0]: %u", res1->words[0]);
            elog(INFO, "words [1]: %u", res1->words[1]);
            elog(INFO, "-------------\r\n");

            //select oid from pg_catalog.pg_constraint;
            //57358
            elog(INFO, "constraintOid: %d", constraintOid);

            //contypid ???
            elog(INFO, "rd_pkindex: %d", trigdata->tg_relation->rd_pkindex);

            Bitmapset *res2 = trigdata->tg_relation->rd_pkattr;
            elog(INFO, "*rd_pkattr: %d", res2);
            // elog(INFO, "*rd_pkattr nwords: %d", res2->nwords);


            write_data_to_file();
            */

            //TODO
            elog(INFO, "FirstLowInvalidHeapAttributeNumber: %d", FirstLowInvalidHeapAttributeNumber);
            int i = -1;
            while ((i = bms_next_member(pkattnos , i)) >= 0) {
                /* do stuff with i */
                /* you'll need to use i - FirstLowInvalidHeapAttributeNumber to get the pg_attribute.attnum */
                int col_idx = i + FirstLowInvalidHeapAttributeNumber;
                elog(INFO, "primary key col_idx: %d", col_idx);
            }
        } else {
            elog(INFO, "get_primary_key_attnos result NULL");
        }



        // SPI_finish();
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
