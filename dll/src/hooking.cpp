#include <Windows.h>
#include "Xorstr.h"
#include "native.h"
#include "hooking.h"
#include "offsets.h"
#include "MinHook.h"
#include "native_invoke.h"
#include "cheats.h"

using namespace native;

GameCombatManager::_tMasterDamagePlayer oMasterDamagePlayer;
void hMasterDamagePlayer(
    GameCombatManager* __this, 
    PhotonPlayer* player, 
    int damage, 
    bool ignoreShield, 
    int* damageResult,
    int* shieldEffect,
    const MethodInfo* method_info
) {
    if(player != PhotonNetwork::get_player(nullptr)) {
        oMasterDamagePlayer(__this, player, damage, ignoreShield, damageResult, shieldEffect, method_info);
        return;
    }

    *damageResult = 1;
    *shieldEffect = 0;
}

CodeStage::AntiCheat::Detectors::RRCheatDetector::_tUpdate oUpdate;
void hUpdate(CodeStage::AntiCheat::Detectors::RRCheatDetector* __this, const MethodInfo* method_info) {
    native_invoke::run();
    oUpdate(__this, method_info);
}

void dead() {}

RecRoom::Core::Locomotion::PlayerMovement::_tAddFlyEnabled oAddFlyEnabled;
void hAddFlyEnabled(RecRoom::Core::Locomotion::PlayerMovement* __this, bool enable, Il2CppObject* token, int priority, const MethodInfo* method_info) {
    oAddFlyEnabled(__this, enable ? enable : cheats::internal_persistent_flight_enabled(), token, priority, method_info);
}

Il2CppObject* hLocalConsumeGiftPackage(RecNet::GiftPackage* gift, Il2CppObject* callback, const MethodInfo* method_info) {
    MH_DisableHook(RecNet::Avatars::LocalConsumeGiftPackage);
    return nullptr;
}

SessionManager::_tLocalPlayerSpawned oLocalPlayerSpawned;
void hLocalPlayerSpawned(SessionManager* __this, const MethodInfo* method_info) {
    oLocalPlayerSpawned(__this, method_info);

    cheats::internal_handle_room_join();
    MH_DisableHook(SessionManager::LocalPlayerSpawned);
}

SessionManager::_tJoinRoom oJoinRoom;
Il2CppObject* hJoinRoom(SessionManager* __this, Il2CppString* roomName, Il2CppString* roomSceneName, bool isPrivate, int inviteMode, bool bypassMovementModeRestriction, const MethodInfo* method_info) {
    cheats::internal_handle_room_leave();

    MH_EnableHook(SessionManager::LocalPlayerSpawned);

    return oJoinRoom(__this, roomName, roomSceneName, isPrivate, inviteMode, bypassMovementModeRestriction, method_info);
}

ToolSkinMapper::_tThisTool_PostPickupEvent oThisTool_PostPickupEvent;
void hThisTool_PostPickupEvent(ToolSkinMapper* __this, Tool* thisTool, const MethodInfo* mi) {
    using namespace cheats;

    oThisTool_PostPickupEvent(__this, thisTool, mi);

    auto tool = internal_get_skin_tool(__this->PrefabName);

    if(tool == SkinnableTool::Invalid) return;

    char guid_buf[64];
    if(internal_try_get_skin_guid(tool, Tool::get_authority(thisTool, nullptr), guid_buf)) {
        ToolSkinMapper::ApplySkin(__this, il2cpp_string_new(guid_buf), false, nullptr);
    }
}

bool is_recurring = false;
bool disable = false;
RecRoom::Core::Combat::RangedWeapon::_tFire oFire;
void hFire(RecRoom::Core::Combat::RangedWeapon* __this, UnityEngine::Vector3 velocity, float charge, const MethodInfo* mi) {
    cheats::internal_handle_fire(__this, charge);

    oFire(__this, velocity, charge, mi);
}

RecRoom::Core::Combat::Weapon::_tSetAmmunition oSetAmmunition;
void hSetAmmunition(RecRoom::Core::Combat::Weapon* __this, int mag, int reserve, const MethodInfo* mi) {
    oSetAmmunition(__this, 1, 1, mi);
}

bool hget_IsOnCooldown(void*, void*) {
    return false;
}

bool hooking::init() {
    using namespace native;
    //load the base address
    game_assembly_base = (size_t)GetModuleHandleA(XS("GameAssembly.dll"));

    //fragile version check, assuming the offset won't be used for another method
    auto test = reinterpret_cast<unsigned char*>(game_assembly_base + OFFSET_RECNET_STORAGE_UPLOADFILE);
    if(test[-1] != 0xCC) return false;

    //load method ptrs
    //List
    System::Collections::Generic::List::get_Count = reinterpret_cast<System::Collections::Generic::List::_tget_Count>(game_assembly_base + OFFSET_SYSTEM_COLLECTIONS_GENERIC_LIST_GET_COUNT);
    System::Collections::Generic::List::get_Item = reinterpret_cast<System::Collections::Generic::List::_tget_Item>(game_assembly_base + OFFSET_SYSTEM_COLLECTIONS_GENERIC_LIST_GET_ITEM);

    //GameObject
    UnityEngine::GameObject::GetComponentInChildren = reinterpret_cast<UnityEngine::GameObject::_tGetComponentInChildren>(game_assembly_base + OFFSET_UNITYENGINE_GAMEOBJECT_GETCOMPONENTINCHILDREN);
    UnityEngine::GameObject::GetComponentsInChildren = reinterpret_cast<UnityEngine::GameObject::_tGetComponentsInChildren>(game_assembly_base + OFFSET_UNITYENGINE_GAMEOBJECT_GETCOMPONENTSINCHILDREN);
    UnityEngine::GameObject::get_transform = reinterpret_cast<UnityEngine::GameObject::_tget_transform>(game_assembly_base + OFFSET_UNITYENGINE_GAMEOBJECT_GET_TRANSFORM);
    UnityEngine::GameObject::AddComponent = reinterpret_cast<UnityEngine::GameObject::_tAddComponent>(game_assembly_base + OFFSET_UNITYENGINE_GAMEOBJECT_ADDCOMPONENT);

    //Component
    UnityEngine::Component::get_transform = reinterpret_cast<UnityEngine::Component::_tget_transform>(game_assembly_base + OFFSET_UNITYENGINE_COMPONENT_GET_TRANSFORM);
    UnityEngine::Component::get_gameObject = reinterpret_cast<UnityEngine::Component::_tget_gameObject>(game_assembly_base + OFFSET_UNITYENGINE_COMPONENT_GET_GAMEOBJECT);

    //Transform
    UnityEngine::Transform::get_position = reinterpret_cast<UnityEngine::Transform::_tget_position>(game_assembly_base + OFFSET_UNITYENGINE_TRANSFORM_GET_POSITION);
    UnityEngine::Transform::set_position = reinterpret_cast<UnityEngine::Transform::_tset_position>(game_assembly_base + OFFSET_UNITYENGINE_TRANSFORM_SET_POSITION);
    UnityEngine::Transform::get_forward = reinterpret_cast<UnityEngine::Transform::_tget_forward>(game_assembly_base + OFFSET_UNITYENGINE_TRANSFORM_GET_FORWARD);
    UnityEngine::Transform::get_rotation = reinterpret_cast<UnityEngine::Transform::_tget_rotation>(game_assembly_base + OFFSET_UNITYENGINE_TRANSFORM_GET_ROTATION);
    UnityEngine::Transform::set_localScale = reinterpret_cast<UnityEngine::Transform::_tset_localScale>(game_assembly_base + OFFSET_UNITYENGINE_TRANSFORM_SET_LOCALSCALE);
    UnityEngine::Transform::GetChild = reinterpret_cast<UnityEngine::Transform::_tGetChild>(game_assembly_base + OFFSET_UNITYENGINE_TRANSFORM_GETCHILD);
    UnityEngine::Transform::SetParent = reinterpret_cast<UnityEngine::Transform::_tSetParent>(game_assembly_base + OFFSET_UNITYENGINE_TRANSFORM_SETPARENT_O);

    //Quaternion
    UnityEngine::Quaternion::get_eulerAngles = reinterpret_cast<UnityEngine::Quaternion::_tget_eulerAngles>(game_assembly_base + OFFSET_UNITYENGINE_QUATERNION_GET_EULERANGLES);
    UnityEngine::Quaternion::set_eulerAngles = reinterpret_cast<UnityEngine::Quaternion::_tset_eulerAngles>(game_assembly_base + OFFSET_UNITYENGINE_QUATERNION_SET_EULERANGLES);

    //Renderer
    UnityEngine::Renderer::set_enabled = reinterpret_cast<UnityEngine::Renderer::_tset_enabled>(game_assembly_base + OFFSET_UNITYENGINE_RENDERER_SET_ENABLED);

    //Storage
    RecNet::Storage::UploadFile = reinterpret_cast<RecNet::Storage::_tUploadFile>(game_assembly_base + OFFSET_RECNET_STORAGE_UPLOADFILE);

    //Avatars
    RecNet::Avatars::LocalConsumeGiftPackage = reinterpret_cast<RecNet::Avatars::_tLocalConsumeGiftPackage>(game_assembly_base + OFFSET_RECNET_AVATARS_LOCALCONSUMEGIFTPACKAGE);
    
    //RRCheatDetector
    CodeStage::AntiCheat::Detectors::RRCheatDetector::Update = reinterpret_cast<CodeStage::AntiCheat::Detectors::RRCheatDetector::_tUpdate>(game_assembly_base + OFFSET_CODESTAGE_ANTICHEAT_DETECTORS_RRCHEATDETECTOR_UPDATE);

    //MonoBehaviour
    Photon::MonoBehaviour::get_photonView = reinterpret_cast<Photon::MonoBehaviour::_tget_photonView>(game_assembly_base + OFFSET_PHOTON_MONOBEHAVIOUR_GET_PHOTONVIEW);
    Photon::MonoBehaviour::get_authority = reinterpret_cast<Photon::MonoBehaviour::_tget_authority>(game_assembly_base + OFFSET_PHOTON_MONOBEHAVIOUR_GET_AUTHORITY);

    //GameCombatManager
    GameCombatManager::MasterDamagePlayer = reinterpret_cast<GameCombatManager::_tMasterDamagePlayer>(game_assembly_base + OFFSET_GAMECOMBATMANAGER_MASTERDAMAGEPLAYER);

    //PhotonNetwork
    PhotonNetwork::get_isMasterClient = reinterpret_cast<PhotonNetwork::_tget_isMasterClient>(game_assembly_base + OFFSET_PHOTONNETWORK_GET_ISMASTERCLIENT);
    PhotonNetwork::get_player = reinterpret_cast<PhotonNetwork::_tget_player>(game_assembly_base + OFFSET_PHOTONNETWORK_GET_PLAYER);
    PhotonNetwork::Instantiate = reinterpret_cast<PhotonNetwork::_tInstantiate>(game_assembly_base + OFFSET_PHOTONNETWORK_INSTANTIATE);
    PhotonNetwork::RPC = reinterpret_cast<PhotonNetwork::_tRPC>(game_assembly_base + OFFSET_PHOTONNETWORK_RPC);
    PhotonNetwork::RPC_player = reinterpret_cast<PhotonNetwork::_tRPC_player>(game_assembly_base + OFFSET_PHOTONNETWORK_RPC_O);
    PhotonNetwork::Destroy = reinterpret_cast<PhotonNetwork::_tDestroy>(game_assembly_base + OFFSET_PHOTONNETWORK_DESTROY);
    PhotonNetwork::get_otherPlayers = reinterpret_cast<PhotonNetwork::_tget_otherPlayers>(game_assembly_base + OFFSET_PHOTONNETWORK_GET_OTHERPLAYERS);
    PhotonNetwork::get_playerList = reinterpret_cast<PhotonNetwork::_tget_playerList>(game_assembly_base + OFFSET_PHOTONNETWORK_GET_PLAYERLIST);
    PhotonNetwork::DestroyPlayerObjects = reinterpret_cast<PhotonNetwork::_tDestroyPlayerObjects>(game_assembly_base + OFFSET_PHOTONNETWORK_DESTROYPLAYEROBJECTS);

    //PhotonPlayer
    PhotonPlayer::get_name = reinterpret_cast<PhotonPlayer::_tget_name>(game_assembly_base + OFFSET_PHOTONPLAYER_GET_NAME);
    PhotonPlayer::get_ID = reinterpret_cast<PhotonPlayer::_tget_ID>(game_assembly_base + OFFSET_PHOTONPLAYER_GET_ID);

    //Polaroid
    Polaroid::AuthoritySetImageName = reinterpret_cast<Polaroid::_tAuthoritySetImageName>(game_assembly_base + OFFSET_POLAROID_AUTHORITYSETIMAGENAME);
    Polaroid::AuthoritySetImageNameStr = reinterpret_cast<Polaroid::_tAuthoritySetImageNameStr>(game_assembly_base + OFFSET_POLAROID_AUTHORITYSETIMAGENAMEB__35_0);

    //PlayerMovement
    RecRoom::Core::Locomotion::PlayerMovement::AddFlyEnabled = reinterpret_cast<RecRoom::Core::Locomotion::PlayerMovement::_tAddFlyEnabled>(game_assembly_base + OFFSET_RECROOM_CORE_LOCOMOTION_PLAYERMOVEMENT_ADDFLYENABLED);
    RecRoom::Core::Locomotion::PlayerMovement::get_IsFlyingEnabled = reinterpret_cast<RecRoom::Core::Locomotion::PlayerMovement::_tget_IsFlyingEnabled>(game_assembly_base + OFFSET_RECROOM_CORE_LOCOMOTION_PLAYERMOVEMENT_GET_ISFLYINGENABLED);
    
    //RangedWeapon
    RecRoom::Core::Combat::RangedWeapon::Fire = reinterpret_cast<RecRoom::Core::Combat::RangedWeapon::_tFire>(game_assembly_base + OFFSET_RECROOM_CORE_COMBAT_RANGEDWEAPON_FIRE_O);
    RecRoom::Core::Combat::RangedWeapon::get_IsOnCooldown = reinterpret_cast<RecRoom::Core::Combat::RangedWeapon::_tget_IsOnCooldown>(game_assembly_base + OFFSET_RECROOM_CORE_COMBAT_RANGEDWEAPON_GET_ISONCOOLDOWN);

    //Weapon
    RecRoom::Core::Combat::Weapon::SetAmmunition = reinterpret_cast<RecRoom::Core::Combat::Weapon::_tSetAmmunition>(game_assembly_base + OFFSET_RECROOM_CORE_COMBAT_WEAPON_SETAMMUNITION);
    RecRoom::Core::Combat::Weapon::get_HasEnoughMagazineAmmunition = reinterpret_cast<RecRoom::Core::Combat::Weapon::_tget_HasEnoughMagazineAmmunition>(game_assembly_base + OFFSET_RECROOM_CORE_COMBAT_WEAPON_GET_HASENOUGHMAGAZINEAMMUNITION);

    //PlayerGameRoleManager
    RecRoom::Systems::PlayerRoles::PlayerGameRoleManager::get_Instance = reinterpret_cast<RecRoom::Systems::PlayerRoles::PlayerGameRoleManager::_tget_Instance>(game_assembly_base + OFFSET_RECROOM_SYSTEMS_PLAYERROLES_PLAYERGAMEROLEMANAGER_GET_INSTANCE);
    
    //PlayerRoomRoleManager
    RecRoom::Systems::PlayerRoles::PlayerRoomRoleManager::get_Instance = reinterpret_cast<RecRoom::Systems::PlayerRoles::PlayerRoomRoleManager::_tget_Instance>(game_assembly_base + OFFSET_RECROOM_SYSTEMS_PLAYERROLES_PLAYERROOMROLEMANAGER_GET_INSTANCE);

    //Array
    Il2CppArray::GetValue = reinterpret_cast<Il2CppArray::_tGetValue>(game_assembly_base + OFFSET_SYSTEM_ARRAY_GETVALUE);
    Il2CppArray::SetValue = reinterpret_cast<Il2CppArray::_tSetValue>(game_assembly_base + OFFSET_SYSTEM_ARRAY_SETVALUE);

    //Player
    Player::get_PhotonPlayer = reinterpret_cast<Player::_tget_PhotonPlayer>(game_assembly_base + OFFSET_PLAYER_GET_PHOTONPLAYER);
    Player::set_DeveloperDisplayMode = reinterpret_cast<Player::_tset_DeveloperDisplayMode>(game_assembly_base + OFFSET_PLAYER_SET_DEVELOPERDISPLAYMODE);

    //SessionManager
    SessionManager::LocalPlayerSpawned = reinterpret_cast<SessionManager::_tLocalPlayerSpawned>(game_assembly_base + OFFSET_SESSIONMANAGER_LOCALPLAYERSPAWNED);
    SessionManager::JoinRoom = reinterpret_cast<SessionManager::_tJoinRoom>(game_assembly_base + OFFSET_SESSIONMANAGER_JOINROOM);

    //GiftManager
    GiftManager::get_IsAGiftBoxCurrentlySpawned = reinterpret_cast<GiftManager::_tget_IsAGiftBoxCurrentlySpawned>(game_assembly_base + OFFSET_GIFTMANAGER_GET_ISAGIFTBOXCURRENTLYSPAWNED);
    GiftManager::OnGiftPackageReceivedEvent = reinterpret_cast<GiftManager::_tOnGiftPackageReceivedEvent>(game_assembly_base + OFFSET_GIFTMANAGER_ONGIFTPACKAGERECEIVEDEVENT);
    GiftManager::GetPackageVariantByContext = reinterpret_cast<GiftManager::_tGetPackageVariantByContext>(game_assembly_base + OFFSET_GIFTMANAGER_GETPACKAGEVARIANTBYCONTEXT);
    GiftManager::get_DefaultGiftBoxMaterial = reinterpret_cast<GiftManager::_tget_DefaultGiftBoxMaterial>(game_assembly_base + OFFSET_GIFTMANAGER_GET_DEFAULTGIFTBOXMATERIAL);

    //MutableOverridableBool
    RecRoom::Utils::OverridableFields::MutableOverridableBool::SerializeToProtobuf = reinterpret_cast<RecRoom::Utils::OverridableFields::MutableOverridableBool::_tSerializeToProtobuf>(game_assembly_base + OFFSET_RECROOM_UTILS_OVERRIDABLEFIELDS_MUTABLEOVERRIDABLEBOOL_SERIALIZETOPROTOBUF);
    
    //MutableOverridableFloat
    RecRoom::Utils::OverridableFields::MutableOverridableFloat::SerializeToProtobuf = reinterpret_cast<RecRoom::Utils::OverridableFields::MutableOverridableFloat::_tSerializeToProtobuf>(game_assembly_base + OFFSET_RECROOM_UTILS_OVERRIDABLEFIELDS_MUTABLEOVERRIDABLEFLOAT_SERIALIZETOPROTOBUF);
    
    //MutableOverridableInt
    RecRoom::Utils::OverridableFields::MutableOverridableInt::SerializeToProtobuf = reinterpret_cast<RecRoom::Utils::OverridableFields::MutableOverridableInt::_tSerializeToProtobuf>(game_assembly_base + OFFSET_RECROOM_UTILS_OVERRIDABLEFIELDS_MUTABLEOVERRIDABLEINT_SERIALIZETOPROTOBUF);
    
    //MutableOverridableVoteKickType
    RecRoom::Utils::OverridableFields::MutableOverridableVoteKickType::SerializeToProtobuf = reinterpret_cast<RecRoom::Utils::OverridableFields::MutableOverridableVoteKickType::_tSerializeToProtobuf>(game_assembly_base + OFFSET_RECROOM_UTILS_OVERRIDABLEFIELDS_MUTABLEOVERRIDABLEVOTEKICKTYPE_SERIALIZETOPROTOBUF);

    //ToolSkinMapper
    ToolSkinMapper::ApplySkin = reinterpret_cast<ToolSkinMapper::_tApplySkin>(game_assembly_base + OFFSET_TOOLSKINMAPPER_APPLYSKIN);
    ToolSkinMapper::ThisTool_PostPickupEvent = reinterpret_cast<ToolSkinMapper::_tThisTool_PostPickupEvent>(game_assembly_base + OFFSET_TOOLSKINMAPPER_THISTOOL_POSTPICKUPEVENT);
    
    RecRoomSceneManager::get_instance = reinterpret_cast<RecRoomSceneManager::_tget_instance>(game_assembly_base + OFFSET_RECROOMSCENEMANAGER_GET_INSTANCE);

    UnityEngine::AudioClip::Create = reinterpret_cast<UnityEngine::AudioClip::_tCreate>(game_assembly_base + OFFSET_UNITYENGINE_AUDIOCLIP_CREATE);
    UnityEngine::AudioClip::SetData = reinterpret_cast<UnityEngine::AudioClip::_tSetData>(game_assembly_base + OFFSET_UNITYENGINE_AUDIOCLIP_SETDATA);

    PhotonVoiceRecorder::set_DebugEchoMode = reinterpret_cast<PhotonVoiceRecorder::_tset_DebugEchoMode>(game_assembly_base + OFFSET_PHOTONVOICERECORDER_SET_DEBUGECHOMODE);
    PhotonVoiceRecorder::set_Transmit = reinterpret_cast<PhotonVoiceRecorder::_tset_DebugEchoMode>(game_assembly_base + OFFSET_PHOTONVOICERECORDER_SET_TRANSMIT);
    PhotonVoiceRecorder::UpdateAudioSource = reinterpret_cast<PhotonVoiceRecorder::_tUpdateAudioSource>(game_assembly_base + OFFSET_PHOTONVOICERECORDER_UPDATEAUDIOSOURCE);

    return MH_Initialize() == MH_OK &&
        MH_CreateHook(GameCombatManager::MasterDamagePlayer, &hMasterDamagePlayer, (void**)&oMasterDamagePlayer) == MH_OK &&
        MH_CreateHook(CodeStage::AntiCheat::Detectors::RRCheatDetector::Update, &hUpdate, (void**)&oUpdate) == MH_OK &&
        MH_CreateHook(RecRoom::Core::Locomotion::PlayerMovement::AddFlyEnabled, &hAddFlyEnabled, (void**)&oAddFlyEnabled) == MH_OK && 
        MH_CreateHook(RecNet::Avatars::LocalConsumeGiftPackage, &hLocalConsumeGiftPackage, nullptr) == MH_OK &&
        MH_CreateHook(SessionManager::LocalPlayerSpawned, &hLocalPlayerSpawned, (void**)&oLocalPlayerSpawned) == MH_OK &&
        MH_CreateHook(SessionManager::JoinRoom, &hJoinRoom, (void**)&oJoinRoom) == MH_OK &&
        MH_CreateHook(ToolSkinMapper::ThisTool_PostPickupEvent, &hThisTool_PostPickupEvent, (void**)&oThisTool_PostPickupEvent) == MH_OK &&
        MH_CreateHook(RecRoom::Core::Combat::RangedWeapon::Fire, &hFire, (void**)&oFire) == MH_OK &&
        MH_CreateHook(RecRoom::Core::Combat::RangedWeapon::get_IsOnCooldown, &hget_IsOnCooldown, nullptr) == MH_OK &&
        MH_CreateHook(RecRoom::Core::Combat::Weapon::SetAmmunition, &hSetAmmunition, (void**)&oSetAmmunition) == MH_OK &&
        true;
}

bool hooking::destroy() {
    return MH_DisableHook(MH_ALL_HOOKS) == MH_OK &&
        MH_Uninitialize() == MH_OK;
}