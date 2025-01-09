<nav class="navbar navbar-expand-lg bg-body-tertiary">
    <div class="container-fluid">
        <a class="navbar-brand" href="/">Ollama Chat with Database</a>
        <button aria-controls="navbarText" aria-expanded="false" aria-label="Toggle navigation" class="navbar-toggler"
                data-bs-target="#navbarText" data-bs-toggle="collapse" type="button">
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarText">
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                    <a aria-current="page" class="nav-link active" href="/">Home</a>
                </li>
                <li class="nav-item dropdown">
                    <a class="nav-link dropdown-toggle" data-bs-toggle="dropdown" href="#" role="button" aria-expanded="false">Models</a>
                    <ul class="dropdown-menu">
                        <li><a class="dropdown-item" href="/models/listdb">Database Models</a></li>
                        <li><a class="dropdown-item" href="/models/listlocal">Local Models</a></li>
                        <li><a class="dropdown-item" href="/models/running">Locally Running Models</a></li>
                        <li><a class="dropdown-item" href="/models/create">Create Model</a></li>
                    </ul>
                </li>
                <li class="nav-item dropdown">
                    <a class="nav-link dropdown-toggle" data-bs-toggle="dropdown" href="#" role="button" aria-expanded="false">Prompt</a>
                    <ul class="dropdown-menu">
                        <li><a class="dropdown-item" href="/prompt/add">Add Prompt</a></li>
                        <li><a class="dropdown-item" href="/prompt/all">All Prompts</a></li>
                    </ul>
                </li>
                <li class="nav-item dropdown">
                    <a class="nav-link dropdown-toggle" data-bs-toggle="dropdown" href="#" role="button" aria-expanded="false">Chat</a>
                    <ul class="dropdown-menu">
                        <li><a class="dropdown-item" href="/chat/all">All Chats</a></li>
                        <li><a class="dropdown-item" href="/chat/streaming">Live Chat</a></li>
                    </ul>
                </li>

                <li class="nav-item">
                    <a class="nav-link" href="/queue">Queue</a>
                </li>
            </ul>
        </div>
    </div>
</nav>

