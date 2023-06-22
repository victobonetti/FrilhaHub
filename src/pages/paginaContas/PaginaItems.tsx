import { useEffect, useState } from "react"
import Item from "../../interfaces/Item";
import { Link, useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api";
import Account from "../../interfaces/Account";

export default function PaginaItems() {

    const { id } = useParams();

    const [account, setAccount] = useState<Account>()

    useEffect(() => {
        const fetchData = async () => {
            let accountId = id;
            try {
                let data: Account = await invoke('find_account_by_id', { accountId })
                setAccount(data);
            } catch (e) {
            }


        }
        fetchData();
    }, [])

    return (
        <div className=" flex w-full h-full">
            <div className="bg-slate-900 w-3/4 h-full">
                <h2 className=" my-2 w-full text-center text-xl font-semibold text-slate-300">{account?.owner.toUpperCase()}</h2>
                <table className=" w-full">
                    <tbody className=" text-slate-300  w-full table-auto flex flex-col ">
                        <thead className=" select-none bg-slate-400 font-semibold flex w-full text-sm ">
                            <tr className="flex w-full">
                                <td className=" text-sm flex items-center p-2 text-slate-600 w-3/12 ">LANÇADO EM</td>
                                <td className=" text-sm flex items-center p-2 text-slate-600 w-2/12 ">PRODUTO</td>
                                <td className=" text-sm flex items-center p-2 text-slate-600 w-1/12 ">QTD.</td>
                                <td className=" text-sm flex items-center p-2 text-slate-600 w-1/12 ">PREÇO UN.</td>
                                <td className=" text-sm flex items-center p-2 text-slate-600 w-1/12 ">TOTAL</td>
                                <td className=" text-sm flex items-center p-2 text-slate-600 w-2/12 ">OBSERVAÇÕES</td>
                                <td className=" text-sm flex items-center p-2 text-slate-600 w-2/12 ">AÇÕES</td>
                            </tr>
                        </thead>

                        {account && account.items.length < 1 && <tr className=" w-full bg-slate-800 p-4 text-2xl">Não foram encontrados registros.</tr>}

                        {account && account.items?.map((data) => {
                            return (
                                <tr key={String(data.id)} className=" w-full flex justify-evenly bg-slate-800  odd:bg-slate-700">
                                    <td className=" text-sm p-2 text-slate-300 w-3/12 ">{data.created_at}</td>
                                    <td className=" text-sm p-2 text-slate-300 w-2/12 ">{data.name}</td>
                                    <td className=" text-sm p-2 text-slate-300 w-1/12 ">{data.quantity}</td>
                                    <td className=" text-sm p-2 text-slate-300 w-1/12 ">R${data.price.toFixed(2)}</td>
                                    <td className=" text-sm p-2 text-slate-300 w-1/12 ">R${Number(data.quantity * data.price).toFixed(2)}</td>
                                    <td className=" text-sm p-2 text-slate-300 w-2/12 ">{data.notes}</td>
                                    <td className=" text-sm p-2 text-slate-300 w-2/12 ">botao</td>

                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            </div>
            <div className=" border-l-8 border-slate-950 bg-slate-700 w-1/4 h-full flex flex-col  p-4">
                <div className=" shadow-inner bg-slate-800 p-4">
                    <p className=" text-slate-300 text-sm ">Dívida</p> <span className=" text-xl font-semibold text-red-400">R${account && Number(account.account_total - account.paid_amount).toFixed(2)}</span>
                    <p className=" mt-2 text-slate-300 text-sm ">Valor pago:</p> <span className=" text-xl font-semibold text-emerald-400">R${account && Number(account.paid_amount).toFixed(2)}</span>
                    <p className=" mt-2 text-slate-300 text-sm ">Total da conta:</p> <span className=" text-xl font-semibold text-slate-400">R${account && Number(account.account_total).toFixed(2)}</span>
                </div >
                <Link to={`/contas/items/add/${account?.id}`}><button type="submit" className=" w-full mt-4 transition-all hover:bg-transparent hover:text-cyan-300 border border-cyan-300  bg-cyan-300 text-cyan-900 font-semibold px-4 py-2 rounded text-lg">Adicionar item</button></Link>
                <Link className=" w-full text-center" to={'/contas'}><p className=" mt-2 text-slate-400 underline cursor-pointer ml-2">Voltar</p></Link>

            </div >
        </div >
    )
}