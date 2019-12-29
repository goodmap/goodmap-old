use crate::{i18n, Actions, WebAPI, constants::PanelView::PanelView as V, Msg};
use seed::prelude::*;
use std::collections::HashMap;

// TODO: import T from "../constants/ActionTypes";
// TODO: import C from "../constants/Categories";

type UnknownJsType = ();

#[derive(Debug)]
pub struct Mdl {
    pub menu: bool,
    pub left: Option<V>,
    pub right: Option<V>,
    pub modal: Option<V>,
    pub waiting_for_search_results: bool,
    pub explainRatingContext: Option<UnknownJsType>,
    pub selectedContext: Option<UnknownJsType>,
    pub showLeftPanel: bool,
    pub locales: HashMap<i18n::Lang, serde_json::Map<String, serde_json::Value>>,
    pub current_lang: i18n::Lang,
}

impl Default for Mdl {
    fn default() -> Self {
        Mdl {
            menu: false,
            left: Some(V::RESULT),
            right: None,
            modal: None,
            waiting_for_search_results: true,
            explainRatingContext: None,
            selectedContext: None,
            showLeftPanel: true,
            locales: HashMap::new(),
            current_lang: i18n::Lang::En,
        }
    }
}

pub fn update(action: &Msg, state: &mut Mdl, orders: &mut impl Orders<Msg>) {

    match action {
                     Msg::Client(Actions::client::Msg::toggleMenu) |
                     Msg::Client(Actions::client::Msg::toggleLandingPage) => {
                        let m = !state.menu;
                        // TODO: let newView = V[payload];
                        if state.modal.is_none() {
                            state.menu = m;
                            if !m { state.right = None; }
                            if m { state.left = None; }
                        }
                     }

                     Msg::Client(Actions::client::Msg::showMenu) => {
                         state.menu = true;
                         state.right = None;
                     }

                     Msg::Client(Actions::client::Msg::showInfo(info)) => {
                        state.menu = info.is_some();
                        state.right = *info;
                     }

        // TODO:     case T.SHOW_SUBSCRIBE_TO_BBOX:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         menu: false,
        // TODO:         right: null,
        // TODO:         left: V.SUBSCRIBE_TO_BBOX
        // TODO:       }
        // TODO:
        // TODO:     case T.SUBSCRIBE_TO_BBOX_RESULT:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         left: V.RESULT
        // TODO:       }
        // TODO:
        // TODO:     case T.REGISTER_RESULT:
        // TODO:       if (state.right === V.REGISTER) {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           right: V.REGISTER_SUCCESS
        // TODO:         }
        // TODO:       }
        // TODO:       return state
        // TODO:
        // TODO:     case T.LOGIN_RESULT:
        // TODO:       if (action.error) {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           right: V.LOGIN_ERROR
        // TODO:         }
        // TODO:       } else {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           right: V.LOGIN_SUCCESS
        // TODO:         }
        // TODO:       }
        // TODO:
        // TODO:     case T.SHOW_IMPRINT:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         menu: true,
        // TODO:         right: V.IMPRINT
        // TODO:       }
        // TODO:
        // TODO:     case T.SHOW_PRIVACY_STATEMENT:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         menu: true,
        // TODO:         right: V.PRIVACY_STATEMENT
        // TODO:       }
        // TODO:
        // TODO:     case T.SHOW_NEW_ENTRY:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         menu: false,
        // TODO:         left: V.NEW
        // TODO:       }
        // TODO:     case T.SHOW_NEW_RATING:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         menu: false,
        // TODO:         left: V.NEW_RATING,
        // TODO:         explainRatingContext: null,
        // TODO:         selectedContext: null
        // TODO:       }
        // TODO:     case T.EDIT_CURRENT_ENTRY:
        // TODO:       if (!action.error) {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           menu: false,
        // TODO:           left: V.EDIT
        // TODO:         }
        // TODO:       }
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         left: V.IO_ERROR
        // TODO:       }
        // TODO:
        // TODO:     case T.SHOW_IO_WAIT:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         left: V.WAIT
        // TODO:       }
        // TODO:
        // TODO:     case T.CANCEL_NEW:
        // TODO:     case T.CANCEL_WAIT_IO:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         left: V.RESULT
        // TODO:       }
        // TODO:
        // TODO:     case T.CANCEL_RATING:
        // TODO:     case T.CANCEL_EDIT:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         left: V.ENTRY
        // TODO:       }
        // TODO:     case T.NEW_ENTRY_RESULT:
        // TODO:
        // TODO:       if (!action.error) {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           left: V.ENTRY
        // TODO:         }
        // TODO:       }
        // TODO:       return state;
        // TODO:
        // TODO:     case T.SEARCH_RESULT_ENTRIES:
        // TODO:       if (action.error) {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           left: V.IO_ERROR
        // TODO:         }
        // TODO:       } else if (!((state.left != null) || action.noList)) {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           left: V.RESULT
        // TODO:         }
        // TODO:       }
        // TODO:       return state;
        // TODO:
        // TODO:     case T.ENTRIES_RESULT:
        // TODO:     case T.SEARCH_RESULT_EVENTS:
        // TODO:     case T.SEARCH_RESULT_EVENTS_WITHOUT_PLACE:
        // TODO:     case T.NO_SEARCH_RESULTS:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         waiting_for_search_results: false
        // TODO:       }
        // TODO:
        // TODO:     case T.SET_CURRENT_ENTRY:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         left: action.payload != null ? V.ENTRY : V.RESULT,
        // TODO:         menu: false
        // TODO:       }
        // TODO:
        // TODO:     case T.SHOW_MAP:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         menu: false,
        // TODO:       }
        // TODO:
        // TODO:     case T.SHOW_LEFT_PANEL:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         showLeftPanel: true
        // TODO:       }
        // TODO:
        // TODO:     case T.HIDE_LEFT_PANEL:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         showLeftPanel: false
        // TODO:       }
        // TODO:
        // TODO:     case T.SHOW_SEARCH_RESULTS:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         left: V.RESULT,
        // TODO:         waiting_for_search_results: true
        // TODO:       }
        // TODO:
        // TODO:     case T.SET_SEARCH_TIME:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         waiting_for_search_results: true
        // TODO:       }
        // TODO:
        // TODO:     case T.SHOW_OWN_POSITION:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         modal: V.LOCATE
        // TODO:       }
        // TODO:
        // TODO:     case T.CANCEL_OWN_POSITION:
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         modal: null
        // TODO:       }
        // TODO:     case T.OWN_POSITION_RESULT:
        // TODO:
        // TODO:       if (action.payload) {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           modal: null
        // TODO:         }
        // TODO:       } else if (state.modal === V.LOCATE) {
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           modal: V.LOCATE_DISABLED
        // TODO:         }
        // TODO:       }
        // TODO:       return state;
        // TODO:
        // TODO:     case T.SHOW_FEATURE_TO_DONATE:
        // TODO:       if (payload) {
        // TODO:         switch (payload) {
        // TODO:           case "events":
        // TODO:             return {
        // TODO:               ...state,
        // TODO:               modal: V.DONATE_FOR_EVENTS
        // TODO:             }
        // TODO:           default:
        // TODO:             return state;
        // TODO:         }
        // TODO:       }
        // TODO:       return {
        // TODO:         ...state,
        // TODO:         modal: null
        // TODO:       }
        // TODO:
        // TODO:     case T.EXPLAIN_RATING_CONTEXT:
        // TODO:       return{
        // TODO:         ...state,
        // TODO:         explainRatingContext: action.payload
        // TODO:       }
        // TODO:
        // TODO:     case T.EMAIL_CONFIRMATION_RESULT:
        // TODO:       if(action.error){
        // TODO:         return{
        // TODO:           ...state,
        // TODO:           menu: true,
        // TODO:           right: V.CONFIRM_EMAIL_ERROR
        // TODO:         }
        // TODO:       } else{
        // TODO:         return{
        // TODO:           ...state,
        // TODO:           menu: true,
        // TODO:           right: V.EMAIL_CONFIRMED
        // TODO:         }
        // TODO:       }
        // TODO:
        // TODO:     case "@@redux-form/CHANGE":
        // TODO:       if (action.meta.field === "context"){
        // TODO:         return {
        // TODO:           ...state,
        // TODO:           selectedContext: action.payload
        // TODO:         }
        // TODO:       }
        // TODO:       return state;
        // TODO:
        // TODO:     default:
        // TODO:       return state;
        // TODO:   }
       Msg::Client(Actions::client::Msg::FetchLocale(lng)) => {
           orders.perform_cmd(WebAPI::fetch_locale(*lng));
       }
       Msg::Server(Actions::server::Msg::LocaleResult(lang, Ok(d))) => {
           state.locales.insert(*lang, d.clone());
       }
       Msg::Client(Actions::client::Msg::SetLocale(lng)) => {
           if state.locales.get(lng).is_none() {
               orders.perform_cmd(WebAPI::fetch_locale(*lng));
           }
           state.current_lang = *lng;
       }
        _ => {
            // do nothing
        }
    }
}
