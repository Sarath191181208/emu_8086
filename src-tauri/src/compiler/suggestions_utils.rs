
use super::suggestions::SuggestionType;


pub(in super) fn get_org_100h() -> Vec<SuggestionType>{
    vec![SuggestionType::Constant16bit(0x100)]
}