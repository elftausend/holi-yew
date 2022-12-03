use crate::components::Auth;
use yew::prelude::*;

#[function_component(Tos)]
pub fn tos() -> Html {
    html! {
        <Auth>
            <div class = "container-fluid">
                <h1>{"Über "}
                    <span class="et_color">{"h"}</span>
                    <span class="it_color">{"o"}</span>
                    <span class="el_color">{"l"}</span>
                    <span class="me_color">{"i"}</span>
                    <span class="mb_color">{"."}</span>
                    {" & Nutzungsbedingungen"}
                </h1>
                <h5>
                    <p>
                        {r#"Holi ist eine Wissensdatenbank für Tests und Lernunterlagen.
                        Jede Person kann selbst Tests und Mitschriften über das "User Panel" unter dem Menü "Upload" allen anderen Schülern und Schülerinnen zur Verfügung stellen."#}<br/>
                    </p>
                    {r#"Um die Suche nach Beiträgen zu vereinfachen, müssen beim Upload folgende Punkte beachtet werden:"#}
                    <br />
                    <ul>
                        <li>{"Auswahl eines passenden Titels"}</li>
                        <li>{"Konventionen bei der Tagvergabe:"}</li>
                        <ol>
                            <li>{"Klasse Bsp.: 2AFET"}</li>
                            <li>{"Angabe von Lehrperson via Kürzel und Nachname Bsp.: FIMI Fischer"}</li>
                            <li>{"Fach, Kürzel und ausgeschrieben Bsp.: AM Mathematik"}</li>
                            <li>{"Die Tags sollten kurz sein."}</li>
                            <li>{"Weiters passende Tags hinzufügen, die das Thema beschreiben sollen."}</li>
                            {"Überprüfen, ob ähnliche Tags bereits vorhanden sind. Wenn ja, sollten diese ähnlichen Tags ebenfalls angehängt werden."}
                            <li>{"Die Abteilung des Uploaders wird automatisch hinzugefügt."}</li>
                            //{r#"Wenn bspw. eine andere Klasse angegeben werden muss, können nachträglich die Tags unter dem Menü "Edit" geändert werden."#}
                            //<br />
                        </ol>
                        <li>{"Für einen für reguläre Nutzer anonymen Upload sollten Name aus bspw. der Kopfzeile entfernt werden."}</li>
                    </ul>
                </h5>
                
                <h4>{"I: "}</h4>{"Mit dem Verwenden von holi. erklären Sie sich mit allen folgenden Punkten einverstanden."}<br /><br />
                <h4>{"II: "}</h4>{"holi ist ein unabhängiges System, das einen Austausch von Lehrmitteln ermöglichen will. holi agiert als eigene Instanz, trotz Kooperation mit der HTL Hollabrunn und Ihrer Schülervertretung."}<br /><br />
                <h4>{"III: "}</h4>{"(1.) Bei einem Verstoß gegen eine der angeführten Bedingungen behält sich holi vor Beiträge selbständig zu löschen bzw. NutzerInnen auszuschließen."}<br />
                {"(2.) Bei dem Upload rassistischer, menschenfeindlicher oder im allgemeinen diskriminierender Inhalte wird holi neben einer Löschung, die HTL Hollabrunn informieren und entsprechend personenbezogene Daten (Name, Vorname, HTL-ID) weitergeben." }<br />
                {"(3.) Eine Beantragung des Lehrpersonals auf Löschung (auch über die Schülervertretung) wird von holi selbst bearbeitet und darüber entschieden. "}<br /><br />
                <h4>{"IV:"}</h4>{"(1.) Bei jedem Upload müssen Eigentumsrechte berücksichtigt werden. "}<br />
                {"(2.) Der Nutzende ist verantwortlich für alle Dokumente, die er hochlädt."}<br /><br />
                <h4>{"V:"}</h4>{"(1.) Mit einem Upload werden die Eigentumsrechte an die Allgemeinheit abgetreten. "}<br />
                {"(2.) holi hat keine Eigentumsanspruch an den hochgeladenen Dokumenten."}<br /><br />
                <h4>{"VI: "}</h4>{"(1.) holi garantiert nicht die Qualität der Dokumente."}<br />
                {"(2.) holi garantiert keine ständige Verfügbarkeit."}<br /><br/>
        </div>
      </Auth>
    }
}
