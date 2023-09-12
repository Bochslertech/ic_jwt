ADMIN_PRINCIPAL=$(dfx identity get-principal)
ADMIN_ACCOUNTID=$(dfx ledger account-id)
TEST_PRINCIPAL='i3o4q-ljrhf-s4evb-ux72j-qdb6g-wzq66-73nfa-h2k3x-dw7zj-4cxkd-zae'
echo $ADMIN_PRINCIPAL
echo $ADMIN_ACCOUNTID
echo $TEST_PRINCIPAL

# dfx deploy ic_jwt --argument "(record { owner = principal \"$ADMIN_PRINCIPAL\"; jwt_secret = \"some_secret\" })" --mode reinstall

dfx deploy ic_jwt --argument "(record { owner = principal \"$ADMIN_PRINCIPAL\"; jwt_secret = \"some_secret\" })"

dfx canister call aax3a-h4aaa-aaaaa-qaahq-cai generate_jwt

dfx canister call aax3a-h4aaa-aaaaa-qaahq-cai get_user_jwt '(principal "3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe")'

dfx canister call aax3a-h4aaa-aaaaa-qaahq-cai set_jwt_secret '("3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe")'

dfx canister call aax3a-h4aaa-aaaaa-qaahq-cai get_jwt_secret

dfx canister call aax3a-h4aaa-aaaaa-qaahq-cai set_owner '(principal "i3o4q-ljrhf-s4evb-ux72j-qdb6g-wzq66-73nfa-h2k3x-dw7zj-4cxkd-zae")'

dfx canister call aax3a-h4aaa-aaaaa-qaahq-cai set_jwt_secret '("3yyxm-t5fpe-v32em-ac6lr-xyort-wuscb-dvl4x-3wnwi-hqkyj-xortw-oqe")'