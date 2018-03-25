/**
 * @file
 *
 * @brief Header for internalnotification plugin
 *
 * @copyright BSD License (see doc/COPYING or http://www.libelektra.org)
 *
 */

#ifndef ELEKTRA_PLUGIN_INTERNALNOTIFICATION_H
#define ELEKTRA_PLUGIN_INTERNALNOTIFICATION_H

#include <kdbnotification.h>
#include <kdbnotificationinternal.h>
#include <kdbplugin.h>

int elektraInternalnotificationGet (Plugin * handle, KeySet * ks, Key * parentKey);
int elektraInternalnotificationSet (Plugin * handle, KeySet * ks, Key * parentKey);
int elektraInternalnotificationClose (Plugin * handle, Key * errorKey);
int elektraInternalnotificationOpen (Plugin * handle, Key * errorKey);

Plugin * ELEKTRA_PLUGIN_EXPORT (internalnotification);

// Not exported by plugin; used for testing
void elektraInternalnotificationUpdateRegisteredKeys (Plugin * plugin, KeySet * keySet);
void elektraInternalnotificationDoUpdate (Key * changedKey, ElektraNotificationCallbackContext * context);

#define INTERNALNOTIFICATION_CHECK_CONVERSION_RANGE(CHECK_RANGE) (*end == 0 && errno == 0 && CHECK_RANGE)
#define INTERNALNOTIFICATION_CHECK_CONVERSION (*end == 0 && errno == 0)

#define INTERNALNOTIFICATION_REGISTER_NAME(TYPE_NAME) elektraInternalnotificationRegister##TYPE_NAME

#define INTERNALNOTIFICATION_EXPORT_FUNCTION(TYPE_NAME)                                                                                    \
	keyNew ("system/elektra/modules/internalnotification/exports/register" #TYPE_NAME, KEY_FUNC,                                       \
		INTERNALNOTIFICATION_REGISTER_NAME (TYPE_NAME), KEY_END)

#endif
