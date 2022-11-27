use crate::components::Auth;
use yew::prelude::*;

#[function_component(Tos)]
pub fn tos() -> Html {
    html! {
        <Auth>
            <div class = "container-fluid">
                <h1>{"Nutzungsbedingungen"}</h1>
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
