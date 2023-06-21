import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import Account from "../../interfaces/Account";
import ConfirmModal from "../../components/ConfirmModal";


export default function PaginaContas({ feedback }: FeedbackProps) {

    const [resposta, setResposta] = useState<Account[]>([]);
    const [toDelete, setToDelete] = useState<Account>();
    const [modalExcluirAberto, setModalExcluirAberto] = useState(false);

    const abrirModalExcluir = (account: Account) => {
        setToDelete(account);
        setModalExcluirAberto(true);
    }

    const excluirConta = async () => {
        let id = toDelete?.id
        try {

            feedback(false, "Usuário excluído com sucesso.")
            fecharModalExcluir();
        }
        catch {
            feedback(true, "Erro ao excluir usuário.")
        }

    }

    const fecharModalExcluir = () => {
        setToDelete(undefined);
        setModalExcluirAberto(false);
    }


    useEffect(() => {
        const fetchData = async (): Promise<void> => {
            try {
                let data: Account[] = await invoke('find_all_accounts', {});
                setResposta(data);

                feedback(false, "Contas encontradas com sucesso.");
            } catch (e) {
                feedback(true, String(e));
            }
        };

        fetchData();

    }, []);

    return (
        <>

            {modalExcluirAberto && <ConfirmModal
                titulo="Tem certeza?"
                texto="Por favor, confirme que deseja prosseguir com a exclusão do usuário clicando no botão abaixo."
                botaotexto="Sim, excluir."
                callbackConfirm={() => excluirConta()}
                callbackCancel={() => fecharModalExcluir()}
            />
            }

            {!modalExcluirAberto &&
                <><div className=" justify-end p-2 flex bg-slate-950 ">
                    <Link to={'/contas/novo'}><button className=" transition-all hover:bg-transparent hover:text-cyan-300 border border-cyan-300  bg-cyan-300 text-cyan-900 font-semibold px-4 py-2 rounded text-lg">Criar nova conta</button></Link>
                </div>
                    <div className=" p-4 flex flex-wrap">
                        {resposta.map((c, i) => {
                            return (
                                <div key={i} className=" text-slate-300 mb-2 mx-1 shadow-lg rounded bg-slate-800 w-1/5 p-2">
                                    <div className=" flex justify-between border-b border-slate-700 pb-1 mb-1">
                                        <h3 className=" text-slate-200 font-semibold ">{c.owner.toUpperCase()}</h3>
                                        {c.account_total - c.paid_amount <= 0 &&<h4 className=" bg-emerald-400 rounded-full text-xs text-emerald-900 font-bold flex items-center justify-center px-1">Quitado</h4>}
                                        {c.account_total - c.paid_amount > 0 &&<h4 className=" bg-neutral-400 rounded-full text-xs text-neutral-900 font-bold flex items-center justify-center px-1">Em aberto</h4>}
                                    </div>
                                    <p className=" text-slate-300 text-xs">Dívida: <span className=" text-red-400">R${Number(c.account_total - c.paid_amount).toFixed(2)}</span></p>
                                    <p className=" text-slate-300 mt-1 text-xs">Valor pago: <span className=" text-emerald-400">R${Number(c.paid_amount).toFixed(2)}</span></p>
                                    <div className=" py-2 flex flex-wrap justify-between "><Link to={`/contas/payments/${c.id}`}><button className=" transition-all hover:bg-transparent hover:text-emerald-300 border border-emerald-300  bg-emerald-300 text-emerald-900 font-semibold px-2 py-1 rounded text-xs ">Adicionar pagamento</button></Link>
                                        <Link to={`/contas/items/${c.id}`}><button className=" transition-all hover:bg-transparent hover:text-blue-300 border border-blue-300  bg-blue-300 text-blue-900 font-semibold px-2 py-1 rounded  text-xs ">Adicionar Itens</button></Link>
                                    </div>
                                </div>
                            )
                        })}

                    </div>
                </>
            }

        </>

    )
}