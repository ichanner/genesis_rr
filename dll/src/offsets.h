#pragma once

#define _OFFSET(Off) (Off - 0x180000000)

constexpr size_t OFFSET_UNITYENGINE_GAMEOBJECT_GETCOMPONENTINCHILDREN = _OFFSET(0x181E39650);
constexpr size_t OFFSET_UNITYENGINE_GAMEOBJECT_GETCOMPONENTSINCHILDREN = _OFFSET(0x181E397A0);
constexpr size_t OFFSET_UNITYENGINE_GAMEOBJECT_GET_TRANSFORM = _OFFSET(0x181E3A650);
constexpr size_t OFFSET_UNITYENGINE_GAMEOBJECT_ADDCOMPONENT = _OFFSET(0x181E391A0);

constexpr size_t OFFSET_UNITYENGINE_COMPONENT_GET_TRANSFORM = _OFFSET(0x181A8C520);
constexpr size_t OFFSET_UNITYENGINE_COMPONENT_GET_GAMEOBJECT = _OFFSET(0x181A8C450);

constexpr size_t OFFSET_UNITYENGINE_TRANSFORM_GET_POSITION = _OFFSET(0x18240A1E0);
constexpr size_t OFFSET_UNITYENGINE_TRANSFORM_SET_POSITION = _OFFSET(0x18240AE80);
constexpr size_t OFFSET_UNITYENGINE_TRANSFORM_GET_FORWARD = _OFFSET(0x182409920);
constexpr size_t OFFSET_UNITYENGINE_TRANSFORM_GET_ROTATION = _OFFSET(0x18240A450);
constexpr size_t OFFSET_UNITYENGINE_TRANSFORM_SET_LOCALSCALE = _OFFSET(0x18240ACD0);
constexpr size_t OFFSET_UNITYENGINE_TRANSFORM_GETCHILD = _OFFSET(0x182406AA0);
constexpr size_t OFFSET_UNITYENGINE_TRANSFORM_SETPARENT_O = _OFFSET(0x182408960);

constexpr size_t OFFSET_UNITYENGINE_QUATERNION_GET_EULERANGLES = _OFFSET(0x18029BEE0);
constexpr size_t OFFSET_UNITYENGINE_QUATERNION_SET_EULERANGLES = _OFFSET(0x18029C060);

constexpr size_t OFFSET_UNITYENGINE_RENDERER_SET_ENABLED = _OFFSET(0x181EF2180);

constexpr size_t OFFSET_UNITYENGINE_AUDIOCLIP_CREATE = _OFFSET(0x182550F70);
constexpr size_t OFFSET_UNITYENGINE_AUDIOCLIP_SETDATA = _OFFSET(0x182551460);

constexpr size_t OFFSET_RECNET_STORAGE_UPLOADFILE = _OFFSET(0x180B57080);

constexpr size_t OFFSET_RECNET_AVATARS_LOCALCONSUMEGIFTPACKAGE = _OFFSET(0x180FF6720);

constexpr size_t OFFSET_PHOTON_MONOBEHAVIOUR_GET_PHOTONVIEW = _OFFSET(0x180E340B0);
constexpr size_t OFFSET_PHOTON_MONOBEHAVIOUR_GET_AUTHORITY = _OFFSET(0x180E33F30);

constexpr size_t OFFSET_GAMECOMBATMANAGER_MASTERDAMAGEPLAYER = _OFFSET(0x18084B520);

constexpr size_t OFFSET_PHOTONNETWORK_GET_ISMASTERCLIENT = _OFFSET(0x180E47970);
constexpr size_t OFFSET_PHOTONNETWORK_GET_OTHERPLAYERS = _OFFSET(0x180E47DD0);
constexpr size_t OFFSET_PHOTONNETWORK_GET_PLAYER = _OFFSET(0x180E47FA0);
constexpr size_t OFFSET_PHOTONNETWORK_GET_PLAYERLIST = _OFFSET(0x180E47E80);
constexpr size_t OFFSET_PHOTONNETWORK_INSTANTIATE = _OFFSET(0x180E40A90);
constexpr size_t OFFSET_PHOTONNETWORK_RPC = _OFFSET(0x180E42D60);
constexpr size_t OFFSET_PHOTONNETWORK_RPC_O = _OFFSET(0x180E42B00);
constexpr size_t OFFSET_PHOTONNETWORK_DESTROY = _OFFSET(0x180E3E9E0);
constexpr size_t OFFSET_PHOTONNETWORK_DESTROYPLAYEROBJECTS = _OFFSET(0x180E3E640);

constexpr size_t OFFSET_PHOTONPLAYER_GET_NAME = _OFFSET(0x180315BC0);
constexpr size_t OFFSET_PHOTONPLAYER_GET_ID = _OFFSET(0x1803404F0);

constexpr size_t OFFSET_POLAROID_AUTHORITYSETIMAGENAME = _OFFSET(0x180A238C0);
constexpr size_t OFFSET_POLAROID_AUTHORITYSETIMAGENAMEB__35_0 = _OFFSET(0x180A250A0);

constexpr size_t OFFSET_CODESTAGE_ANTICHEAT_DETECTORS_RRCHEATDETECTOR_UPDATE = _OFFSET(0x1815E52E0);

constexpr size_t OFFSET_RECROOM_CORE_LOCOMOTION_PLAYERMOVEMENT_ADDFLYENABLED = _OFFSET(0x181343020);
constexpr size_t OFFSET_RECROOM_CORE_LOCOMOTION_PLAYERMOVEMENT_GET_ISFLYINGENABLED = _OFFSET(0x18134E730);

constexpr size_t OFFSET_RECROOM_SYSTEMS_PLAYERROLES_PLAYERGAMEROLEMANAGER_GET_INSTANCE = _OFFSET(0x180B8C480);

constexpr size_t OFFSET_RECROOM_SYSTEMS_PLAYERROLES_PLAYERROOMROLEMANAGER_GET_INSTANCE = _OFFSET(0x180B8F380);

constexpr size_t OFFSET_RECROOM_UTILS_OVERRIDABLEFIELDS_MUTABLEOVERRIDABLEBOOL_SERIALIZETOPROTOBUF = _OFFSET(0x1809FEE60);

constexpr size_t OFFSET_RECROOM_UTILS_OVERRIDABLEFIELDS_MUTABLEOVERRIDABLEFLOAT_SERIALIZETOPROTOBUF = _OFFSET(0x1809FF660);

constexpr size_t OFFSET_RECROOM_UTILS_OVERRIDABLEFIELDS_MUTABLEOVERRIDABLEINT_SERIALIZETOPROTOBUF = _OFFSET(0x1809FFA20);

constexpr size_t OFFSET_RECROOM_UTILS_OVERRIDABLEFIELDS_MUTABLEOVERRIDABLEVOTEKICKTYPE_SERIALIZETOPROTOBUF = _OFFSET(0x180B88B40);

constexpr size_t OFFSET_SYSTEM_ARRAY_GETVALUE = _OFFSET(0x1804D4BF0);
constexpr size_t OFFSET_SYSTEM_ARRAY_SETVALUE = _OFFSET(0x1804D61D0);

constexpr size_t OFFSET_PLAYER_GET_PHOTONPLAYER = _OFFSET(0x18094AF40);
constexpr size_t OFFSET_PLAYER_SET_DEVELOPERDISPLAYMODE = _OFFSET(0x18094BD40);

constexpr size_t OFFSET_GIFTMANAGER_GET_ISAGIFTBOXCURRENTLYSPAWNED = _OFFSET(0x180AA0ED0);
constexpr size_t OFFSET_GIFTMANAGER_ONGIFTPACKAGERECEIVEDEVENT = _OFFSET(0x180A9FFA0);
constexpr size_t OFFSET_GIFTMANAGER_GETPACKAGEVARIANTBYCONTEXT = _OFFSET(0x180A9FA00);
constexpr size_t OFFSET_GIFTMANAGER_GET_DEFAULTGIFTBOXMATERIAL = _OFFSET(0x1804422C0);

constexpr size_t OFFSET_SESSIONMANAGER_LOCALPLAYERSPAWNED = _OFFSET(0x1807ABE70);
constexpr size_t OFFSET_SESSIONMANAGER_JOINROOM = _OFFSET(0x1807AB740);

constexpr size_t OFFSET_TOOLSKINMAPPER_APPLYSKIN = _OFFSET(0x180E28690);
constexpr size_t OFFSET_TOOLSKINMAPPER_THISTOOL_POSTPICKUPEVENT = _OFFSET(0x180E29FE0);

constexpr size_t OFFSET_RECROOM_CORE_COMBAT_RANGEDWEAPON_FIRE_O = _OFFSET(0x180EC8A40);
constexpr size_t OFFSET_RECROOM_CORE_COMBAT_RANGEDWEAPON_GET_ISONCOOLDOWN = _OFFSET(0x180ECA460);

constexpr size_t OFFSET_RECROOM_CORE_COMBAT_WEAPON_GET_HASENOUGHMAGAZINEAMMUNITION = _OFFSET(0x180ED41E0);
constexpr size_t OFFSET_RECROOM_CORE_COMBAT_WEAPON_SETAMMUNITION = _OFFSET(0x180ED20E0);

constexpr size_t OFFSET_RECROOMSCENEMANAGER_GET_INSTANCE = _OFFSET(0x180D85100);

constexpr size_t OFFSET_PHOTONVOICERECORDER_SET_DEBUGECHOMODE = _OFFSET(0x180939B10);
constexpr size_t OFFSET_PHOTONVOICERECORDER_SET_TRANSMIT = _OFFSET(0x180939DF0);
constexpr size_t OFFSET_PHOTONVOICERECORDER_UPDATEAUDIOSOURCE = _OFFSET(0x1809380E0);

constexpr size_t OFFSET_SYSTEM_COLLECTIONS_GENERIC_LIST_GET_COUNT = _OFFSET(0x180310AE0);
constexpr size_t OFFSET_SYSTEM_COLLECTIONS_GENERIC_LIST_GET_ITEM = _OFFSET(0x182201AD0);