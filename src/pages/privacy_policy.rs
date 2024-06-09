use yew::prelude::*;

pub struct PrivacyPolicyPage {}

impl Component for PrivacyPolicyPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let company = "Interpretica Lda";
        html! {
            <>
            <div class="section container">
                <h1 class="title">{ "Isabelle Privacy Policy" }</h1>

                <div class="content">
                    <p>
                        { "This privacy policy will explain how "} { company } {" uses the personal data we
                           collect from you when you use our management system (\"website\" later in the
                           text)." }
                    </p>


                    <h2 class="title">{ "What data do we collect" }</h2>
                    <p>
                        { company } { " collects the following data:"}
                    </p>
                    <ul>
                        <li>
                            { "Personal identification information (Name, email address, phone number,
                               fiscal number, etc.)"}
                        </li>
                        <li>
                            { "Training data (time, teacher, associated horse)"}
                        </li>
                    </ul>

                    <h2 class="title">{ "How do we collect your data?" }</h2>
                    <p>
                        { "You directly provide " } { company } { " with most of the data we collect. We
                           collect data and process data when you:" }
                    </p>
                    <ul>
                        <li>
                            { "Register at the Equestrian Club of your choice providing the personal
                               information to them." }
                        </li>
                        <li>
                            { "Register online in order to use our service." }
                        </li>
                        <li>
                            { "Use or view our website via your browser’s cookies." }
                        </li>
                    </ul>

                    <h2 class="title">{ "How will we use your data?" }</h2>
                    <p>
                        { company } { " collects your data so that we can:" }
                    </p>
                    <ul>
                        <li>
                            { "Manage your account." }
                        </li>
                        <li>
                            { "Process your training process and notify you about the changes via E-Mail or
                               Google Calendar changes." }
                        </li>
                    </ul>

                    <p>
                        { "When " } { company } { " processes your training information, it may send your
                          E-Mail and training time to Google in order to synchronize calendar between you
                          and your teacher." }
                    </p>


                    <h2 class="title">{ "How do we store your data?" }</h2>
                    <p>
                        { company } { " securely stores your data at Digital Ocean LLC servers in one
                        of supported European Union locations (Amsterdam, Netherlands; Frankfurt,
                        Germany; etc). We've taken measures to ensure that your data is safe:" }
                    </p>
                    <ul>
                        <li>
                            { "Your account's password is securely stored using industrial-strength
                               cryptographic algorithms." }
                        </li>
                        <li>
                            { "The servers' firewalls are configured to have zero additional end points
                               except ones required for Web Interface functionality." }
                        </li>
                        <li>
                            { "Server software is regularly updated in order not to let hackers use
                              vulnerabilities." }
                        </li>
                        <li>
                            { "Server data is regularly backed up using Digital Ocean LLC cloud service." }
                        </li>
                    </ul>

                    <p>
                        { company } { " will keep your account information for the time until you
                        cancel your subscription to Sport Club of your choice. When you decide to delete
                        your account, we will delete your data securely." }
                    </p>

                    <h2 class="title">{ "Marketing" }</h2>
                    <p>
                        { company } { " wouldn't send any marketing information to you." }
                    </p>

                    <h2 class="title">{ "What are your data protection rights?" }</h2>
                    <p>
                        { company } { " would like to make sure you are fully aware of all of your data
                        protection rights. Every user is entitled to the following:" }
                    </p>
                    <ul>
                        <li>
                            <strong>{ "The right to access" }</strong>
                            { " – You have the right to request " } { company } { " for
                            copies of your personal data. We may charge you a small fee for this service." }
                        </li>
                        <li>
                            <strong>{ "The right to rectification" }</strong>
                            { " – You have the right to request " } { company }
                            { "correct any information you believe is inaccurate. You also
                              have the right to request " } { company } {" to complete the information you
                              believe is incomplete." }
                        </li>
                        <li>
                            <strong>{ "The right to erasure" }</strong>
                            { " – You have the right to request that "} { company }
                            { " erase your personal data, under certain conditions." }
                        </li>
                        <li>
                            <strong>{ "The right to restrict processing" }</strong>
                            { " – You have the right to request that " }
                              { company } { " restrict the processing of your personal data, under certain
                            conditions." }
                        </li>
                        <li>
                            <strong>{ "The right to object to processing" }</strong>
                            { " – You have the right to object to " }
                            { company } { "’s processing of your personal data, under certain conditions."}
                        </li>
                        <li>
                            <strong>{ "The right to data portability" }</strong>
                            { " – You have the right to request that " }
                            { company } {" transfer the data that we have collected to another
                              organization, or directly to you, under certain conditions." }
                        </li>
                    </ul>

                    <p>
                        { "If you make a request, we have one month to respond to you. If you would like
                          to exercise any of these rights, please contact us at our email: " }
                        <a href="mailto:gdpr@interpretica.io">{"gdpr@interpretica.io"}</a>
                    </p>


                    <h2 class="title">{ "Cookies" }</h2>
                    <p>
                        { "Cookies are text files placed on your computer to collect standard Internet log
                          information and visitor behavior information. When you visit our websites, we
                          may collect information from you automatically through cookies or similar
                          technology" }
                        { "For further information, visit " }
                        <a href="https://allaboutcookies.org">{"allaboutcookies.org"}</a>
                        { "." }
                    </p>

                    <h2 class="title">{ "How do we use cookies?" }</h2>
                    <p>
                        { company } { " uses cookies in a range of ways to improve your experience on
                          our website, including:" }
                    </p>
                    <ul>
                        <li>
                            { "Keeping you signed in." }
                        </li>
                    </ul>

                    <h2 class="title">{ "What types of cookies do we use?" }</h2>
                    <p>
                        { "There is a number of different types of cookies, however, our website uses:" }
                    </p>
                    <ul>
                        <li>
                            { "Functionality – "} { company } { " uses these cookies so that we recognize you
                              on our website and remember your previously selected preferences. These
                              could include what language you prefer and location you are in. Only
                              first-party cookies are used." }
                        </li>
                    </ul>

                    <h2 class="title">{ "How to manage cookies" }</h2>
                    <p>
                        { "You can set your browser not to accept cookies, and the above website tells you
                           how to remove cookies from your browser. However, in a few cases, some of our
                           website features may not function as a result." }
                    </p>

                    <h2 class="title">{ "Privacy policies of other websites" }</h2>
                    <p>
                        { "The " } { company } { " website contains links to other websites. Our privacy
                          policy applies only to our website, so if you click on a link to another
                          website, you should read their privacy policy."}
                    </p>

                    <h2 class="title">{ "Changes to our privacy policy" }</h2>
                    <p>
                        { company } { " keeps its privacy policy under regular review and places any
                          updates on this web page. This privacy policy was last updated on 10 November
                          2023."}
                    </p>

                    <h2 class="title">{ "How to contact us" }</h2>
                    <p>
                        { "If you have any questions about "} { company } { "’s privacy policy, the data we
                          hold on you, or you would like to exercise one of your data protection rights,
                          please do not hesitate to contact us." }
                    </p>

                    <p>
                        { "E-Mail us at: " }<a href="mailto:gdpr@interpretica.io">{ "gdpr@interpretica.io" }</a>
                    </p>

                    <h2 class="title">{ "How to contact the appropriate authorities" }</h2>
                    <p>
                        { "Should you wish to report a complaint or if you feel that " } { company }
                        { " has not addressed your concern in a satisfactory manner, you may contact the
                          Information Commissioner’s Office." }
                    </p>

                    <p>
                        { "Site: " }<a href="https://www.cnpd.pt">{ "https://www.cnpd.pt" }</a>
                    </p>
                    <p>
                        { "E-Mail us at: " }<a href="mailto:geral@cnpd.pt">{ "geral@cnpd.pt" }</a>
                    </p>
                    <p>
                        { "Address: Av. D. Carlos I, 134, 1º 1200-651 Lisboa" }
                    </p>
                </div>
            </div>
            </>
        }
    }
}
