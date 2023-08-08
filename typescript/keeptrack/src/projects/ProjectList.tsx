import React from 'react'
import { Project } from './Project';

interface ProjectListProps {
    projects: Project[];
}

function ProjectList({ projects }: ProjectListProps) {
  return (
    // <ul className="row">
    //   {projects.map((project) => (
    //     <li key={project.id}>{project.name}</li>
    //   ))}
    // </ul>
    <div className="row">
      {projects.map((project) => (
        <div key={project.id} className='cols-sm'>
          <div className="card">
            <img src={project.imageUrl} alt={project.name}></img>
            <section className='section dark'>
              <h5 className='strong'>
                <strong>{project.name}</strong>
              </h5>
              <p>{project.description}</p>
              <p>Budget : {project.budget.toLocaleString()}</p>
            </section>
          </div>
        </div>
      ))}
    </div>
  )
  
}

export default ProjectList